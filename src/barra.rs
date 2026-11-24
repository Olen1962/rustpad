#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::texto_sobre;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{Config, Textos};
use crate::editar::AcaoEditar;
use crate::estilo::{self, cor32};
use crate::ficheiro::AcaoFicheiro;
use crate::ver::AcaoVer;

pub struct ResultadoBarra {
    pub ficheiro: AcaoFicheiro,
    pub editar: AcaoEditar,
    pub ver: AcaoVer,
    pub alternar_procura: bool,
    pub abrir_preferencias: bool,
    pub fechar_documento: bool,
    pub sair: bool,
}

impl Default for ResultadoBarra {
    fn default() -> Self {
        Self {
            ficheiro: AcaoFicheiro::Nenhuma,
            editar: AcaoEditar::Nenhuma,
            ver: AcaoVer::Nenhuma,
            alternar_procura: false,
            abrir_preferencias: false,
            fechar_documento: false,
            sair: false,
        }
    }
}

#[derive(Clone)]
enum IconeCarregado {
    Textura(egui::TextureHandle),
    Svg(String, bool),
    Ausente,
}

#[derive(Clone, Copy)]
enum IconeVetorial {
    Novo,
    Abrir,
    Guardar,
    GuardarComo,
    Desfazer,
    Refazer,
    Procurar,
    NumerosLinha,
    QuebraLinha,
    BarraEstado,
    ZoomMais,
    ZoomMenos,
    Preferencias,
}

#[derive(Default)]
pub struct IconCache {
    pasta_atual: String,
    geracao: u64,
    proximo_id: u64,
    entradas: HashMap<String, IconeCarregado>,
}

impl IconCache {
    pub fn invalidar(&mut self) {
        self.entradas.clear();
        self.pasta_atual.clear();
        self.geracao = self.geracao.wrapping_add(1);
    }

    fn obter(&mut self, ctx: &egui::Context, pasta_base: &str, nomes: &[&str]) -> IconeCarregado {
        if self.pasta_atual != pasta_base {
            self.entradas.clear();
            self.pasta_atual = pasta_base.to_owned();
            self.geracao = self.geracao.wrapping_add(1);
        }
        let chave = format!("{pasta_base}::{}", nomes.first().copied().unwrap_or(""));
        if let Some(cacheado) = self.entradas.get(&chave) {
            return cacheado.clone();
        }

        self.proximo_id = self.proximo_id.wrapping_add(1);
        let uri_svg = format!("bytes://rustpad_g{}_i{}.svg", self.geracao, self.proximo_id);
        let carregado = match procurar_icone_sistema(Path::new(pasta_base), nomes) {
            Some(caminho) => carregar_arquivo_icone(ctx, &chave, &uri_svg, &caminho),
            None => IconeCarregado::Ausente,
        };
        self.entradas.insert(chave, carregado.clone());
        carregado
    }
}

fn carregar_arquivo_icone(
    ctx: &egui::Context,
    chave: &str,
    uri_svg: &str,
    caminho: &Path,
) -> IconeCarregado {
    let Ok(bytes) = fs::read(caminho) else {
        return IconeCarregado::Ausente;
    };
    if bytes.is_empty() {
        return IconeCarregado::Ausente;
    }

    let ext = caminho
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if ext == "svg" {
        let simbolico = caminho
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.contains("-symbolic"));
        ctx.include_bytes(uri_svg.to_owned(), bytes);
        return IconeCarregado::Svg(uri_svg.to_owned(), simbolico);
    }

    if let Ok(img) = image::load_from_memory(&bytes) {
        let rgba = img.to_rgba8();
        if rgba.width() == 0 || rgba.height() == 0 {
            return IconeCarregado::Ausente;
        }
        let tamanho = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        let color_img = egui::ColorImage::from_rgba_unmultiplied(tamanho, &pixels);
        let tex = ctx.load_texture(chave, color_img, egui::TextureOptions::LINEAR);
        return IconeCarregado::Textura(tex);
    }

    IconeCarregado::Ausente
}

fn procurar_icone_sistema(base: &Path, nomes: &[&str]) -> Option<PathBuf> {
    if !base.is_dir() {
        return None;
    }

    let mut candidatos_dir = vec![base.to_path_buf()];

    let subdirs = [
        "24x24/actions",
        "22x22/actions",
        "32x32/actions",
        "16x16/actions",
        "48x48/actions",
        "scalable/actions",
        "symbolic/actions",
        "actions/24",
        "actions/22",
        "actions/32",
        "actions/16",
        "actions/scalable",
        "actions/symbolic",
        "actions",
        "24x24/apps",
        "scalable/apps",
        "24x24/categories",
        "scalable/categories",
        "symbolic/categories",
    ];
    for sub in subdirs {
        let d = base.join(sub);
        if d.is_dir() {
            candidatos_dir.push(d);
        }
    }

    // Se o utilizador indicou diretamente .../24x24/actions, também verifica pastas irmãs na raiz do tema
    if let Some(pai_tamanho) = base.parent() {
        for irma in ["apps", "categories", "status"] {
            let d = pai_tamanho.join(irma);
            if d.is_dir() {
                candidatos_dir.push(d);
            }
        }
        if let Some(raiz_tema) = pai_tamanho.parent() {
            for rel in ["scalable/actions", "32x32/actions", "22x22/actions", "16x16/actions", "24x24/categories", "scalable/categories"] {
                let d = raiz_tema.join(rel);
                if d.is_dir() {
                    candidatos_dir.push(d);
                }
            }
        }
    }

    let exts = ["png", "svg"];
    for nome in nomes {
        for dir in &candidatos_dir {
            for ext in exts {
                let candidato = dir.join(format!("{nome}.{ext}"));
                if candidato.is_file() {
                    return Some(candidato);
                }
                let simbolico = dir.join(format!("{nome}-symbolic.{ext}"));
                if simbolico.is_file() {
                    return Some(simbolico);
                }
            }
        }
    }

    for nome in nomes {
        if let Some(p) = procurar_recursivo(base, nome, 0) {
            return Some(p);
        }
    }
    None
}

fn procurar_recursivo(dir: &Path, nome: &str, prof: u8) -> Option<PathBuf> {
    if prof > 3 {
        return None;
    }
    let entradas = fs::read_dir(dir).ok()?;
    let nome_sym = format!("{nome}-symbolic");
    for entrada in entradas.flatten() {
        let path = entrada.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if stem == nome || stem == nome_sym {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if ext.eq_ignore_ascii_case("png") || ext.eq_ignore_ascii_case("svg") {
                        return Some(path);
                    }
                }
            }
        } else if path.is_dir() {
            if let Some(achado) = procurar_recursivo(&path, nome, prof + 1) {
                return Some(achado);
            }
        }
    }
    None
}

fn desenhar_icone_vetorial(
    painter: &egui::Painter,
    rect: egui::Rect,
    tipo: IconeVetorial,
    cor: egui::Color32,
) {
    let s = egui::Stroke::new(1.5_f32, cor);
    let c = rect.center();
    let r = rect.shrink(2.0_f32);
    match tipo {
        IconeVetorial::Novo => {
            painter.rect_stroke(r, 1.5_f32, s);
            painter.line_segment([egui::pos2(c.x, r.top() + 3.0_f32), egui::pos2(c.x, r.bottom() - 3.0_f32)], s);
            painter.line_segment([egui::pos2(r.left() + 3.0_f32, c.y), egui::pos2(r.right() - 3.0_f32, c.y)], s);
        }
        IconeVetorial::Abrir => {
            let pasta = egui::Rect::from_min_max(egui::pos2(r.left(), r.top() + 3.0_f32), r.max);
            painter.rect_stroke(pasta, 1.5_f32, s);
            painter.line_segment([r.left_top(), egui::pos2(c.x, r.top())], s);
        }
        IconeVetorial::Guardar => {
            painter.rect_stroke(r, 1.5_f32, s);
            let topo = egui::Rect::from_min_max(egui::pos2(r.left() + 2.5_f32, r.top()), egui::pos2(r.right() - 2.5_f32, c.y));
            painter.rect_stroke(topo, 0.5_f32, s);
        }
        IconeVetorial::GuardarComo => {
            painter.rect_stroke(r.translate(egui::vec2(-1.5_f32, -1.5_f32)), 1.5_f32, s);
            painter.line_segment([egui::pos2(c.x, c.y), r.right_bottom()], s);
        }
        IconeVetorial::Desfazer => {
            painter.line_segment([egui::pos2(r.left(), c.y), egui::pos2(r.right(), c.y)], s);
            painter.line_segment([egui::pos2(r.left(), c.y), egui::pos2(r.left() + 4.5_f32, c.y - 4.0_f32)], s);
            painter.line_segment([egui::pos2(r.left(), c.y), egui::pos2(r.left() + 4.5_f32, c.y + 4.0_f32)], s);
        }
        IconeVetorial::Refazer => {
            painter.line_segment([egui::pos2(r.left(), c.y), egui::pos2(r.right(), c.y)], s);
            painter.line_segment([egui::pos2(r.right(), c.y), egui::pos2(r.right() - 4.5_f32, c.y - 4.0_f32)], s);
            painter.line_segment([egui::pos2(r.right(), c.y), egui::pos2(r.right() - 4.5_f32, c.y + 4.0_f32)], s);
        }
        IconeVetorial::Procurar => {
            let centro_lupa = egui::pos2(c.x - 1.5_f32, c.y - 1.5_f32);
            painter.circle_stroke(centro_lupa, r.width() * 0.32_f32, s);
            painter.line_segment([egui::pos2(c.x + 1.5_f32, c.y + 1.5_f32), r.right_bottom()], s);
        }
        IconeVetorial::NumerosLinha => {
            for dy in [-4.0_f32, 0.0_f32, 4.0_f32] {
                painter.line_segment([egui::pos2(r.left(), c.y + dy), egui::pos2(r.left() + 3.0_f32, c.y + dy)], s);
                painter.line_segment([egui::pos2(r.left() + 5.5_f32, c.y + dy), egui::pos2(r.right(), c.y + dy)], s);
            }
        }
        IconeVetorial::QuebraLinha => {
            painter.line_segment([egui::pos2(r.left(), r.top() + 2.0_f32), egui::pos2(r.right(), r.top() + 2.0_f32)], s);
            painter.line_segment([egui::pos2(r.left(), c.y + 2.0_f32), egui::pos2(r.right() - 2.0_f32, c.y + 2.0_f32)], s);
            painter.line_segment([egui::pos2(r.left(), c.y + 2.0_f32), egui::pos2(r.left() + 3.5_f32, r.bottom())], s);
        }
        IconeVetorial::BarraEstado => {
            painter.rect_stroke(r, 1.5_f32, s);
            painter.line_segment([egui::pos2(r.left(), r.bottom() - 4.0_f32), egui::pos2(r.right(), r.bottom() - 4.0_f32)], s);
        }
        IconeVetorial::ZoomMais => {
            painter.circle_stroke(c, r.width() * 0.42_f32, s);
            painter.line_segment([egui::pos2(c.x - 3.0_f32, c.y), egui::pos2(c.x + 3.0_f32, c.y)], s);
            painter.line_segment([egui::pos2(c.x, c.y - 3.0_f32), egui::pos2(c.x, c.y + 3.0_f32)], s);
        }
        IconeVetorial::ZoomMenos => {
            painter.circle_stroke(c, r.width() * 0.42_f32, s);
            painter.line_segment([egui::pos2(c.x - 3.0_f32, c.y), egui::pos2(c.x + 3.0_f32, c.y)], s);
        }
        IconeVetorial::Preferencias => {
            painter.circle_stroke(c, r.width() * 0.40_f32, s);
            painter.circle_stroke(c, r.width() * 0.16_f32, s);
        }
    }
}

fn separador_vertical(ui: &mut egui::Ui, cor: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(7.0_f32, 20.0_f32), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let x = rect.center().x;
        ui.painter().line_segment(
            [
                egui::pos2(x, rect.top() + 2.0_f32),
                egui::pos2(x, rect.bottom() - 2.0_f32),
            ],
            egui::Stroke::new(1.0_f32, cor),
        );
    }
}

fn botao_icone(
    ui: &mut egui::Ui,
    cache: &mut IconCache,
    pasta_icones: &str,
    nomes: &[&str],
    vetorial: IconeVetorial,
    dica: &str,
    ativo: bool,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(26.0_f32, 26.0_f32), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let visuais = ui.style().interact_selectable(&resp, ativo);
        ui.painter()
            .rect(rect, visuais.rounding, visuais.bg_fill, visuais.bg_stroke);

        let area_icone = rect.shrink(3.0_f32);
        let uv = egui::Rect::from_min_max(
            egui::pos2(0.0_f32, 0.0_f32),
            egui::pos2(1.0_f32, 1.0_f32),
        );

        match cache.obter(ui.ctx(), pasta_icones, nomes) {
            IconeCarregado::Textura(tex) => {
                ui.painter()
                    .image(tex.id(), area_icone, uv, egui::Color32::WHITE);
            }
            IconeCarregado::Svg(uri, simbolico) => {
                match ui.ctx().try_load_texture(
                    &uri,
                    egui::TextureOptions::LINEAR,
                    egui::SizeHint::default(),
                ) {
                    Ok(egui::load::TexturePoll::Ready { texture }) => {
                        let cor = if simbolico {
                            visuais.fg_stroke.color
                        } else {
                            egui::Color32::WHITE
                        };
                        ui.painter().image(texture.id, area_icone, uv, cor);
                    }
                    Ok(egui::load::TexturePoll::Pending { .. }) | Err(_) => {
                        desenhar_icone_vetorial(
                            ui.painter(),
                            rect.shrink(4.0_f32),
                            vetorial,
                            visuais.fg_stroke.color,
                        );
                    }
                }
            }
            IconeCarregado::Ausente => {
                desenhar_icone_vetorial(
                    ui.painter(),
                    rect.shrink(4.0_f32),
                    vetorial,
                    visuais.fg_stroke.color,
                );
            }
        }
    }
    resp.on_hover_text(dica)
}

pub fn desenhar(
    ctx: &egui::Context,
    cache: &mut IconCache,
    titulo: &str,
    config: &mut Config,
) -> ResultadoBarra {
    let mut res = ResultadoBarra::default();
    let tema = estilo::resolver_tema(config.modo_tema);
    let t = Textos::de(config.idioma);

    // Só desenha barra de título personalizada se as decorações nativas do XFCE estiverem desligadas.
    // Mesmo nesse caso, a barra de título NUNCA contém os menus.
    if !config.decoracoes_sistema {
        let frame_titulo = egui::Frame {
            inner_margin: egui::Margin::symmetric(10.0_f32, 6.0_f32),
            outer_margin: egui::Margin::ZERO,
            rounding: egui::Rounding::ZERO,
            shadow: egui::epaint::Shadow::NONE,
            fill: cor32(tema.barra_titulo),
            stroke: egui::Stroke::new(1.0_f32, cor32(tema.separador)),
        };

        egui::TopBottomPanel::top("barra_titulo")
            .frame(frame_titulo)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let area_arrasto = ui.available_rect_before_wrap();
                    if area_arrasto.width() > 120.0_f32 && area_arrasto.height() > 0.0_f32 {
                        let resp = ui.interact(
                            egui::Rect::from_min_max(
                                area_arrasto.min,
                                egui::pos2(
                                    (area_arrasto.max.x - 108.0_f32).max(area_arrasto.min.x),
                                    area_arrasto.max.y.max(area_arrasto.min.y),
                                ),
                            ),
                            ui.id().with("arrastar_janela"),
                            egui::Sense::click_and_drag(),
                        );
                        if resp.drag_started() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }
                        if resp.double_clicked() {
                            let max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
                        }
                    }

                    let cor_titulo = cor32(texto_sobre(tema.barra_titulo));
                    ui.label(
                        egui::RichText::new(titulo)
                            .strong()
                            .color(cor_titulo),
                    );

                    if ui.available_width() > 60.0_f32 {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let fechar = egui::Button::new(
                                egui::RichText::new("✕").color(cor32(texto_sobre(tema.perigo))),
                            )
                            .fill(cor32(tema.perigo));
                            if ui.add(fechar).clicked() {
                                res.sair = true;
                            }
                            if ui
                                .add(egui::Button::new(egui::RichText::new("□").color(cor_titulo)).frame(false))
                                .clicked()
                            {
                                let max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
                            }
                            if ui
                                .add(egui::Button::new(egui::RichText::new("—").color(cor_titulo)).frame(false))
                                .clicked()
                            {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                            }
                        });
                    }
                });
            });
    }

    // Barra de menus separada da barra de título (estilo XFCE / GTK tradicional)
    let frame_menu = egui::Frame {
        inner_margin: egui::Margin::symmetric(8.0_f32, 4.0_f32),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::ZERO,
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.fundo_janela),
        stroke: egui::Stroke::new(1.0_f32, cor32(tema.separador)),
    };

    egui::TopBottomPanel::top("barra_menu")
        .frame(frame_menu)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(t.menu_ficheiro, |ui| {
                    if ui.button(format!("{} (Ctrl+N)", t.novo)).clicked() {
                        res.ficheiro = AcaoFicheiro::Novo;
                        ui.close_menu();
                    }
                    if ui.button(format!("{} (Ctrl+O)", t.abrir)).clicked() {
                        res.ficheiro = AcaoFicheiro::PedirAbrir;
                        ui.close_menu();
                    }
                    if ui.button(format!("{} (Ctrl+S)", t.guardar)).clicked() {
                        res.ficheiro = AcaoFicheiro::Guardar;
                        ui.close_menu();
                    }
                    if ui.button(t.guardar_como).clicked() {
                        res.ficheiro = AcaoFicheiro::PedirGuardarComo;
                        ui.close_menu();
                    }
                    if ui.button(t.fechar).clicked() {
                        res.fechar_documento = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(format!("{} (Ctrl+Q)", t.sair)).clicked() {
                        res.sair = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button(t.menu_editar, |ui| {
                    if ui.button(format!("{} (Ctrl+Z)", t.desfazer)).clicked() {
                        res.editar = AcaoEditar::Desfazer;
                        ui.close_menu();
                    }
                    if ui.button(format!("{} (Ctrl+Y)", t.refazer)).clicked() {
                        res.editar = AcaoEditar::Refazer;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(format!("{} (Ctrl+F)", t.procurar_substituir)).clicked() {
                        res.alternar_procura = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(format!("{} (Ctrl+,)", t.preferencias)).clicked() {
                        res.abrir_preferencias = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button(t.menu_ver, |ui| {
                    if ui.checkbox(&mut config.numeros_linha, t.numeros_linha).changed() {
                        res.ver = AcaoVer::GuardarConfig;
                    }
                    if ui.checkbox(&mut config.quebra_linha, t.quebra_linha).changed() {
                        res.ver = AcaoVer::GuardarConfig;
                    }
                    if ui.checkbox(&mut config.barra_estado, t.barra_estado).changed() {
                        res.ver = AcaoVer::GuardarConfig;
                    }
                    ui.separator();
                    if ui.button(t.zoom_mais).clicked() {
                        res.ver = AcaoVer::ZoomMais;
                        ui.close_menu();
                    }
                    if ui.button(t.zoom_menos).clicked() {
                        res.ver = AcaoVer::ZoomMenos;
                        ui.close_menu();
                    }
                });
            });
        });

    desenhar_toolbar(ctx, cache, config, &mut res);
    res
}

fn desenhar_toolbar(
    ctx: &egui::Context,
    cache: &mut IconCache,
    config: &mut Config,
    res: &mut ResultadoBarra,
) {
    let tema = estilo::resolver_tema(config.modo_tema);
    let t = Textos::de(config.idioma);
    let pasta = config.pasta_icones.clone();

    let frame = egui::Frame {
        inner_margin: egui::Margin::symmetric(8.0_f32, 5.0_f32),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::ZERO,
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.fundo_cartao),
        stroke: egui::Stroke::new(1.0_f32, cor32(tema.separador)),
    };

    egui::TopBottomPanel::top("toolbar_rustpad")
        .frame(frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(4.0_f32, 4.0_f32);
                ui.spacing_mut().button_padding = egui::vec2(4.0_f32, 4.0_f32);

                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["document-new", "gtk-new", "filenew", "stock_new-text"],
                    IconeVetorial::Novo,
                    t.novo,
                    false,
                )
                .clicked()
                {
                    res.ficheiro = AcaoFicheiro::Novo;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["document-open", "gtk-open", "fileopen", "stock_open"],
                    IconeVetorial::Abrir,
                    t.abrir,
                    false,
                )
                .clicked()
                {
                    res.ficheiro = AcaoFicheiro::PedirAbrir;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["document-save", "gtk-save", "filesave", "stock_save"],
                    IconeVetorial::Guardar,
                    t.guardar,
                    false,
                )
                .clicked()
                {
                    res.ficheiro = AcaoFicheiro::Guardar;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["document-save-as", "gtk-save-as", "filesaveas", "stock_save-as"],
                    IconeVetorial::GuardarComo,
                    t.guardar_como,
                    false,
                )
                .clicked()
                {
                    res.ficheiro = AcaoFicheiro::PedirGuardarComo;
                }

                separador_vertical(ui, cor32(tema.separador));

                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["edit-undo", "gtk-undo", "undo", "stock_undo"],
                    IconeVetorial::Desfazer,
                    t.desfazer,
                    false,
                )
                .clicked()
                {
                    res.editar = AcaoEditar::Desfazer;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["edit-redo", "gtk-redo", "redo", "stock_redo"],
                    IconeVetorial::Refazer,
                    t.refazer,
                    false,
                )
                .clicked()
                {
                    res.editar = AcaoEditar::Refazer;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["edit-find-replace", "edit-find", "gtk-find-and-replace", "gtk-find", "stock_search-and-replace", "stock_search"],
                    IconeVetorial::Procurar,
                    t.procurar_substituir,
                    false,
                )
                .clicked()
                {
                    res.alternar_procura = true;
                }

                separador_vertical(ui, cor32(tema.separador));

                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["format-list-ordered", "view-list-details", "view-list-text", "gtk-index", "format-indent-more"],
                    IconeVetorial::NumerosLinha,
                    t.numeros_linha,
                    config.numeros_linha,
                )
                .clicked()
                {
                    config.numeros_linha = !config.numeros_linha;
                    res.ver = AcaoVer::GuardarConfig;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["format-justify-fill", "format-text-direction-ltr", "gtk-justify-fill", "view-wrap"],
                    IconeVetorial::QuebraLinha,
                    t.quebra_linha,
                    config.quebra_linha,
                )
                .clicked()
                {
                    config.quebra_linha = !config.quebra_linha;
                    res.ver = AcaoVer::GuardarConfig;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["preferences-desktop-display", "view-fullscreen", "gtk-fullscreen", "window-new"],
                    IconeVetorial::BarraEstado,
                    t.barra_estado,
                    config.barra_estado,
                )
                .clicked()
                {
                    config.barra_estado = !config.barra_estado;
                    res.ver = AcaoVer::GuardarConfig;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["zoom-in", "gtk-zoom-in", "stock_zoom-in"],
                    IconeVetorial::ZoomMais,
                    t.zoom_mais,
                    false,
                )
                .clicked()
                {
                    res.ver = AcaoVer::ZoomMais;
                }
                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["zoom-out", "gtk-zoom-out", "stock_zoom-out"],
                    IconeVetorial::ZoomMenos,
                    t.zoom_menos,
                    false,
                )
                .clicked()
                {
                    res.ver = AcaoVer::ZoomMenos;
                }

                separador_vertical(ui, cor32(tema.separador));

                if botao_icone(
                    ui,
                    cache,
                    &pasta,
                    &["preferences-system", "gtk-preferences", "preferences-desktop", "gtk-properties"],
                    IconeVetorial::Preferencias,
                    t.preferencias,
                    false,
                )
                .clicked()
                {
                    res.abrir_preferencias = true;
                }
            });
        });
}
