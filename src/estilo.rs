#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::{CorExt, ModoTema, Rgb, Tema, texto_sobre};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use crate::config::Config;

#[inline]
pub fn cor32(rgb: Rgb) -> egui::Color32 {
    let (r, g, b) = rgb.para_rgb_u8();
    egui::Color32::from_rgb(r, g, b)
}

#[inline]
pub fn cor32_alfa(rgb: Rgb, alfa: u8) -> egui::Color32 {
    let (r, g, b, a) = rgb.para_rgba_u8(alfa);
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn dir_config() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
}

fn ler_xfconf(canal: &str, propriedade: &str) -> Option<String> {
    let saida = Command::new("xfconf-query")
        .args(["-c", canal, "-p", propriedade])
        .output()
        .ok()?;
    if !saida.status.success() {
        return None;
    }
    let val = String::from_utf8_lossy(&saida.stdout).trim().to_owned();
    if val.is_empty() { None } else { Some(val) }
}

fn ler_xsettings_xml(propriedade: &str) -> Option<String> {
    let caminho = dir_config()?.join("xfce4/xfconf/xfce-perchannel-xml/xsettings.xml");
    let conteudo = fs::read_to_string(caminho).ok()?;
    let marcador = format!("name=\"{propriedade}\"");
    let pos = conteudo.find(&marcador)?;
    let trecho = &conteudo[pos..];
    let fim = trecho.find("/>")?;
    let linha = &trecho[..fim];
    let chave_val = "value=\"";
    let v_ini = linha.find(chave_val)? + chave_val.len();
    let resto = &linha[v_ini..];
    let v_fim = resto.find('"')?;
    let valor = resto[..v_fim].trim();
    if valor.is_empty() { None } else { Some(valor.to_owned()) }
}

fn ler_gtk_settings(chave: &str) -> Option<String> {
    let base = dir_config()?;
    for sub in ["gtk-3.0/settings.ini", "gtk-4.0/settings.ini"] {
        if let Ok(conteudo) = fs::read_to_string(base.join(sub)) {
            for linha in conteudo.lines() {
                if let Some((k, v)) = linha.trim().split_once('=') {
                    if k.trim() == chave {
                        let val = v.trim();
                        if !val.is_empty() {
                            return Some(val.to_owned());
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn detetar_nome_tema_sistema() -> Option<String> {
    if let Ok(env_tema) = env::var("GTK_THEME") {
        let limpo = env_tema.split(':').next().unwrap_or("").trim();
        if !limpo.is_empty() {
            return Some(limpo.to_owned());
        }
    }
    ler_xfconf("xsettings", "/Net/ThemeName")
        .or_else(|| ler_xsettings_xml("ThemeName"))
        .or_else(|| ler_gtk_settings("gtk-theme-name"))
}

pub fn detetar_pasta_icones_xfce() -> Option<String> {
    let nome_tema = ler_xfconf("xsettings", "/Net/IconThemeName")
        .or_else(|| ler_xsettings_xml("IconThemeName"))
        .or_else(|| ler_gtk_settings("gtk-icon-theme-name"));

    if let Some(nome) = nome_tema {
        let mut bases = vec![
            PathBuf::from("/usr/local/share/icons"),
            PathBuf::from("/usr/share/icons"),
        ];
        if let Some(home) = env::var_os("HOME") {
            let h = PathBuf::from(home);
            bases.push(h.join(".icons"));
            bases.push(h.join(".local/share/icons"));
        }
        for base in bases {
            let candidato = base.join(&nome);
            if candidato.is_dir() {
                let acoes_24 = candidato.join("24x24/actions");
                if acoes_24.is_dir() {
                    return Some(acoes_24.display().to_string());
                }
                return Some(candidato.display().to_string());
            }
        }
    }

    for padrao in [
        "/usr/local/share/icons/GartoonRedux/24x24/actions",
        "/usr/local/share/icons/GartoonRedux",
    ] {
        let p = Path::new(padrao);
        if p.is_dir() {
            return Some(p.display().to_string());
        }
    }
    None
}

pub fn detetar_tamanho_menu_xfce() -> Option<f32> {
    let fonte = ler_xfconf("xsettings", "/Gtk/FontName")
        .or_else(|| ler_xsettings_xml("FontName"))
        .or_else(|| ler_gtk_settings("gtk-font-name"))?;
    let ultimo = fonte.split_whitespace().last()?;
    let tam = ultimo.parse::<f32>().ok()?;
    if tam.is_finite() {
        Some((tam + 2.0_f32).clamp(10.0_f32, 26.0_f32))
    } else {
        None
    }
}

fn detetar_prefere_escuro(nome_tema: Option<&str>) -> bool {
    if let Ok(env_tema) = env::var("GTK_THEME") {
        if env_tema.to_ascii_lowercase().contains("dark") {
            return true;
        }
    }
    if let Some(nome) = nome_tema {
        let lower = nome.to_ascii_lowercase();
        if lower.contains("dark") || lower.contains("noir") || lower.contains("escuro") {
            return true;
        }
    }
    if let Some(pref) = ler_gtk_settings("gtk-application-prefer-dark-theme") {
        return pref == "1" || pref.eq_ignore_ascii_case("true");
    }
    false
}

fn parse_hex_rgb(s: &str) -> Option<Rgb> {
    let h = s.trim().trim_end_matches([';', ',', '"']).trim();
    let hex = h.strip_prefix('#')?;
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 12 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let b = u8::from_str_radix(&hex[8..10], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

fn ler_xfwm4_xml_tema() -> Option<String> {
    let caminho = dir_config()?.join("xfce4/xfconf/xfce-perchannel-xml/xfwm4.xml");
    let conteudo = fs::read_to_string(caminho).ok()?;
    let marcador = "name=\"theme\"";
    let pos = conteudo.find(marcador)?;
    let trecho = &conteudo[pos..];
    let fim = trecho.find("/>")?;
    let linha = &trecho[..fim];
    let chave_val = "value=\"";
    let v_ini = linha.find(chave_val)? + chave_val.len();
    let resto = &linha[v_ini..];
    let v_fim = resto.find('"')?;
    let valor = resto[..v_fim].trim();
    if valor.is_empty() { None } else { Some(valor.to_owned()) }
}

fn extrair_cor_xpm(caminho: &Path) -> Option<Rgb> {
    let conteudo = fs::read_to_string(caminho).ok()?;
    let mut candidatas = Vec::new();
    for linha in conteudo.lines().take(64) {
        if let Some(pos) = linha.find('#') {
            let token = linha[pos..]
                .split(|c: char| c.is_whitespace() || c == '"' || c == ',')
                .next()
                .unwrap_or("");
            if let Some(rgb) = parse_hex_rgb(token) {
                candidatas.push(rgb);
            }
        }
    }
    if candidatas.is_empty() {
        return None;
    }
    candidatas
        .iter()
        .copied()
        .find(|&(r, g, b)| (r, g, b) != (0, 0, 0) && (r, g, b) != (255, 255, 255))
        .or_else(|| candidatas.first().copied())
}

fn detetar_barra_titulo_xfce() -> Option<Rgb> {
    let wm_tema = ler_xfwm4_xml_tema().or_else(|| ler_xfconf("xfwm4", "/general/theme"))?;
    let mut raizes = vec![
        PathBuf::from("/usr/local/share/themes"),
        PathBuf::from("/usr/share/themes"),
    ];
    if let Some(home) = env::var_os("HOME") {
        let h = PathBuf::from(home);
        raizes.push(h.join(".themes"));
        raizes.push(h.join(".local/share/themes"));
    }

    for raiz in raizes {
        let dir_wm = raiz.join(&wm_tema).join("xfwm4");
        if !dir_wm.is_dir() {
            continue;
        }
        for arq in ["title-1-active.xpm", "title-2-active.xpm", "top-left-active.xpm"] {
            if let Some(rgb) = extrair_cor_xpm(&dir_wm.join(arq)) {
                return Some(rgb);
            }
        }
        if let Ok(themerc) = fs::read_to_string(dir_wm.join("themerc")) {
            let mut texto_claro = false;
            for linha in themerc.lines() {
                if let Some((k, v)) = linha.trim().split_once('=') {
                    let chave = k.trim();
                    if chave == "active_color_1" || chave == "active_color_2" {
                        if let Some(rgb) = parse_hex_rgb(v) {
                            return Some(rgb);
                        }
                    }
                    if chave == "active_text_color" {
                        if let Some(rgb) = parse_hex_rgb(v) {
                            let brilho = 0.299_f32 * rgb.0 as f32
                                + 0.587_f32 * rgb.1 as f32
                                + 0.114_f32 * rgb.2 as f32;
                            if brilho > 160.0_f32 {
                                texto_claro = true;
                            }
                        }
                    }
                }
            }
            if texto_claro {
                return Some((44, 44, 44));
            }
        }
    }

    let lower = wm_tema.to_ascii_lowercase();
    if lower.contains("dark") || lower.contains("noir") || lower.starts_with("default") {
        return Some((44, 44, 44));
    }
    None
}

fn ler_cores_gtk_css(nome_tema: &str) -> HashMap<String, Rgb> {
    let mut mapa = HashMap::new();
    let mut raizes = vec![
        PathBuf::from("/usr/share/themes"),
        PathBuf::from("/usr/local/share/themes"),
    ];
    if let Some(home) = env::var_os("HOME") {
        let h = PathBuf::from(home);
        raizes.push(h.join(".themes"));
        raizes.push(h.join(".local/share/themes"));
    }

    for raiz in raizes {
        for sub in ["gtk-3.0/gtk.css", "gtk-4.0/gtk.css"] {
            let ficheiro = raiz.join(nome_tema).join(sub);
            if let Ok(conteudo) = fs::read_to_string(ficheiro) {
                for linha in conteudo.lines() {
                    let l = linha.trim();
                    if let Some(resto) = l.strip_prefix("@define-color ") {
                        let mut partes = resto.splitn(2, char::is_whitespace);
                        if let (Some(nome), Some(valor)) = (partes.next(), partes.next()) {
                            if let Some(rgb) = parse_hex_rgb(valor) {
                                mapa.insert(nome.trim().to_owned(), rgb);
                            }
                        }
                    }
                }
                if !mapa.is_empty() {
                    return mapa;
                }
            }
        }
    }
    mapa
}

fn resolver_tema_sistema_once() -> Tema {
    let nome_tema = detetar_nome_tema_sistema();
    let escuro = detetar_prefere_escuro(nome_tema.as_deref());
    let mut tema = if escuro { Tema::ESCURO } else { Tema::SISTEMA };
    tema.modo = ModoTema::Sistema;

    if let Some(ref nome) = nome_tema {
        let cores = ler_cores_gtk_css(nome);
        if let Some(&c) = cores.get("theme_bg_color").or_else(|| cores.get("window_bg_color")) {
            tema.fundo_janela = c;
        }
        if let Some(&c) = cores.get("theme_base_color").or_else(|| cores.get("view_bg_color")) {
            tema.fundo_texto = c;
            tema.fundo_cartao = c.misturar(tema.fundo_janela, 0.4_f32);
        }
        if let Some(&c) = cores.get("headerbar_bg_color") {
            tema.barra_titulo = c;
        }
        if let Some(&c) = cores.get("theme_fg_color").or_else(|| cores.get("window_fg_color")) {
            tema.texto_principal = c;
        }
        if let Some(&c) = cores.get("theme_selected_bg_color").or_else(|| cores.get("accent_bg_color")) {
            tema.destaque = c;
            tema.destaque_hover = c.misturar((255, 255, 255), 0.12_f32);
            tema.destaque_clique = c.misturar((0, 0, 0), 0.15_f32);
        }
        if let Some(&c) = cores.get("borders") {
            tema.borda = c;
            tema.separador = c.misturar(tema.fundo_janela, 0.35_f32);
        }
        if let Some(&c) = cores.get("popover_bg_color") {
            tema.popover = c;
        }
    }

    if let Some(barra_xfce) = detetar_barra_titulo_xfce() {
        tema.barra_titulo = barra_xfce;
    }
    tema
}

pub fn resolver_tema(modo: ModoTema) -> Tema {
    static CACHE_SISTEMA: OnceLock<Tema> = OnceLock::new();
    match modo {
        ModoTema::Claro => Tema::CLARO,
        ModoTema::Escuro => Tema::ESCURO,
        ModoTema::Sistema => *CACHE_SISTEMA.get_or_init(resolver_tema_sistema_once),
    }
}

pub fn aplicar_fontes(ctx: &egui::Context, config: &Config) {
    let mut estilo = (*ctx.style()).clone();
    let corpo = config.tamanho_fonte.clamp(9.0_f32, 32.0_f32);
    // Respeita sempre o tamanho configurado pelo utilizador, mesmo no Tema do Sistema.
    let menu = config.tamanho_menu.clamp(10.0_f32, 26.0_f32);

    estilo.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new((menu * 0.85_f32).max(9.0_f32), egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(menu, egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(menu, egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(menu * 1.2_f32, egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(corpo, egui::FontFamily::Monospace),
    );

    ctx.set_style(estilo);
}

pub fn aplicar_tema(ctx: &egui::Context, config: &Config) {
    let tema = resolver_tema(config.modo_tema);
    let (btn_inativo, btn_hover, btn_ativo) = tema.estados_botao();
    let raio = egui::Rounding::same(tema.raio_widget as f32);

    let brilho_janela = 0.299_f32 * tema.fundo_janela.0 as f32
        + 0.587_f32 * tema.fundo_janela.1 as f32
        + 0.114_f32 * tema.fundo_janela.2 as f32;
    let mut visuals = if brilho_janela < 128.0_f32 {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    visuals.override_text_color = Some(cor32(tema.texto_principal));
    visuals.window_fill = cor32(tema.fundo_janela);
    visuals.panel_fill = cor32(tema.fundo_janela);
    visuals.extreme_bg_color = cor32(tema.fundo_texto);
    visuals.faint_bg_color = cor32(tema.fundo_cartao);
    visuals.code_bg_color = cor32(tema.fundo_texto);
    visuals.hyperlink_color = cor32(tema.destaque);
    visuals.warn_fg_color = cor32(tema.aviso);
    visuals.error_fg_color = cor32(tema.perigo);
    visuals.window_rounding = egui::Rounding::same(tema.raio_janela as f32);
    visuals.window_stroke = egui::Stroke::new(1.0_f32, cor32(tema.borda));
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::Vec2::new(0.0_f32, 6.0_f32),
        blur: 20.0_f32,
        spread: 0.0_f32,
        color: cor32_alfa((0, 0, 0), 96),
    };
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: egui::Vec2::new(0.0_f32, 3.0_f32),
        blur: 10.0_f32,
        spread: 0.0_f32,
        color: cor32_alfa((0, 0, 0), 64),
    };

    visuals.selection.bg_fill = cor32(tema.destaque);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, cor32(texto_sobre(tema.destaque)));

    visuals.widgets.noninteractive.bg_fill = cor32(tema.fundo_janela);
    visuals.widgets.noninteractive.weak_bg_fill = cor32(tema.fundo_cartao);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.separador));
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.texto_principal));
    visuals.widgets.noninteractive.rounding = raio;

    visuals.widgets.inactive.bg_fill = cor32(btn_inativo);
    visuals.widgets.inactive.weak_bg_fill = cor32(tema.fundo_cartao);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.borda));
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.texto_principal));
    visuals.widgets.inactive.rounding = raio;

    visuals.widgets.hovered.bg_fill = cor32(btn_hover);
    visuals.widgets.hovered.weak_bg_fill = cor32(btn_hover);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.destaque_hover));
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5_f32, cor32(tema.texto_principal));
    visuals.widgets.hovered.rounding = raio;

    visuals.widgets.active.bg_fill = cor32(btn_ativo);
    visuals.widgets.active.weak_bg_fill = cor32(tema.destaque_clique);
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5_f32, cor32(tema.destaque));
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5_f32, cor32(texto_sobre(tema.destaque)));
    visuals.widgets.active.rounding = raio;

    visuals.widgets.open.bg_fill = cor32(tema.popover);
    visuals.widgets.open.weak_bg_fill = cor32(tema.popover);
    visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.borda));
    visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0_f32, cor32(tema.texto_principal));
    visuals.widgets.open.rounding = raio;

    let mut estilo = (*ctx.style()).clone();
    estilo.visuals = visuals;
    estilo.spacing.item_spacing = egui::vec2(8.0_f32, 6.0_f32);
    estilo.spacing.button_padding = egui::vec2(12.0_f32, 5.0_f32);
    estilo.spacing.window_margin = egui::Margin::same(12.0_f32);
    ctx.set_style(estilo);
}

#[derive(Clone, Copy)]
pub enum IconeCabecalho {
    Preferencias,
    Procura,
    Ficheiro,
    Erro,
}

pub fn frame_dialogo(tema: &Tema) -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::ZERO,
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::same(tema.raio_janela as f32),
        shadow: egui::epaint::Shadow {
            offset: egui::Vec2::new(0.0_f32, 8.0_f32),
            blur: 24.0_f32,
            spread: 0.0_f32,
            color: cor32_alfa((0, 0, 0), 96),
        },
        fill: cor32(tema.fundo_janela),
        stroke: egui::Stroke::new(1.0_f32, cor32(tema.borda)),
    }
}

pub fn cabecalho_dialogo(
    ui: &mut egui::Ui,
    icone: IconeCabecalho,
    titulo: &str,
    tema: &Tema,
) -> bool {
    let mut clicou_fechar = false;
    let r = tema.raio_janela as f32;
    let resp_frame = egui::Frame {
        inner_margin: egui::Margin::symmetric(14.0_f32, 10.0_f32),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding {
            nw: r,
            ne: r,
            sw: 0.0_f32,
            se: 0.0_f32,
        },
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.barra_titulo),
        stroke: egui::Stroke::NONE,
    }
    .show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.set_min_width(ui.available_width());
            let (rect_ic, _) =
                ui.allocate_exact_size(egui::vec2(16.0_f32, 16.0_f32), egui::Sense::hover());
            if ui.is_rect_visible(rect_ic) {
                let cor_ic = match icone {
                    IconeCabecalho::Erro => cor32(tema.perigo),
                    _ => cor32(tema.destaque),
                };
                let s = egui::Stroke::new(1.6_f32, cor_ic);
                let c = rect_ic.center();
                let area = rect_ic.shrink(1.5_f32);
                let p = ui.painter();
                match icone {
                    IconeCabecalho::Preferencias => {
                        p.circle_stroke(c, 4.8_f32, s);
                        p.circle_stroke(c, 1.8_f32, s);
                        for (dx, dy) in [(0.0_f32, -6.5_f32), (0.0_f32, 6.5_f32), (-6.5_f32, 0.0_f32), (6.5_f32, 0.0_f32)] {
                            p.line_segment(
                                [egui::pos2(c.x + dx * 0.72_f32, c.y + dy * 0.72_f32), egui::pos2(c.x + dx, c.y + dy)],
                                s,
                            );
                        }
                    }
                    IconeCabecalho::Procura => {
                        p.circle_stroke(egui::pos2(c.x - 1.5_f32, c.y - 1.5_f32), 4.5_f32, s);
                        p.line_segment(
                            [egui::pos2(c.x + 1.8_f32, c.y + 1.8_f32), egui::pos2(area.right(), area.bottom())],
                            s,
                        );
                    }
                    IconeCabecalho::Ficheiro => {
                        let pasta = egui::Rect::from_min_max(
                            egui::pos2(area.left(), area.top() + 2.5_f32),
                            area.max,
                        );
                        p.rect_stroke(pasta, 1.5_f32, s);
                        p.line_segment([area.left_top(), egui::pos2(c.x, area.top())], s);
                    }
                    IconeCabecalho::Erro => {
                        p.circle_stroke(c, 6.0_f32, s);
                        p.line_segment(
                            [egui::pos2(c.x, c.y - 3.2_f32), egui::pos2(c.x, c.y + 0.8_f32)],
                            s,
                        );
                        p.circle_filled(egui::pos2(c.x, c.y + 3.2_f32), 1.0_f32, cor_ic);
                    }
                }
            }

            let cor_titulo = cor32(texto_sobre(tema.barra_titulo));
            ui.label(
                egui::RichText::new(titulo)
                    .strong()
                    .color(cor_titulo),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    egui::RichText::new("✕").color(cor_titulo),
                )
                .frame(false);
                if ui.add(btn).clicked() {
                    clicou_fechar = true;
                }
            });
        });
    });

    let rect = resp_frame.response.rect;
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.0_f32, cor32(tema.separador)),
    );
    clicou_fechar
}

pub fn frame_rodape_dialogo(tema: &Tema) -> egui::Frame {
    let r = tema.raio_janela as f32;
    egui::Frame {
        inner_margin: egui::Margin::symmetric(14.0_f32, 10.0_f32),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding {
            nw: 0.0_f32,
            ne: 0.0_f32,
            sw: r,
            se: r,
        },
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.fundo_janela),
        stroke: egui::Stroke::NONE,
    }
}

pub fn linha_separadora_topo(ui: &mut egui::Ui, rect: egui::Rect, tema: &Tema) {
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        egui::Stroke::new(1.0_f32, cor32(tema.separador)),
    );
}

pub fn linha_separadora_base(ui: &mut egui::Ui, rect: egui::Rect, tema: &Tema) {
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.0_f32, cor32(tema.separador)),
    );
}

pub fn corpo_dialogo() -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(14.0_f32),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::ZERO,
        shadow: egui::epaint::Shadow::NONE,
        fill: egui::Color32::TRANSPARENT,
        stroke: egui::Stroke::NONE,
    }
}

pub fn cartao_grupo(tema: &Tema) -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::symmetric(12.0_f32, 10.0_f32),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::same((tema.raio_widget + 2) as f32),
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.fundo_cartao),
        stroke: egui::Stroke::new(1.0_f32, cor32(tema.separador)),
    }
}

pub fn titulo_seccao(ui: &mut egui::Ui, texto: &str, tema: &Tema) {
    ui.label(
        egui::RichText::new(texto.to_uppercase())
            .small()
            .strong()
            .color(cor32(tema.texto_secundario)),
    );
}

pub fn botao_segmento(texto: &str, ativo: bool, tema: &Tema) -> egui::Button<'static> {
    let (bg, borda, cor_txt) = if ativo {
        (
            cor32(tema.destaque),
            cor32(tema.destaque),
            cor32(texto_sobre(tema.destaque)),
        )
    } else {
        (
            cor32(tema.fundo_janela),
            cor32(tema.borda),
            cor32(tema.texto_principal),
        )
    };
    let rt = if ativo {
        egui::RichText::new(texto.to_owned()).strong().color(cor_txt)
    } else {
        egui::RichText::new(texto.to_owned()).color(cor_txt)
    };
    egui::Button::new(rt)
        .fill(bg)
        .stroke(egui::Stroke::new(1.0_f32, borda))
        .rounding(egui::Rounding::same(tema.raio_widget as f32))
        .selected(ativo)
}

pub fn linha_checkbox(ui: &mut egui::Ui, texto: &str, valor: &mut bool, largura: f32) -> bool {
    let mut alterou = false;
    ui.allocate_ui_with_layout(
        egui::vec2(largura, 24.0_f32),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_min_width(largura);
            ui.label(texto);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.checkbox(valor, "").changed() {
                    alterou = true;
                }
            });
        },
    );
    alterou
}

pub fn botao_primario(texto: &str, tema: &Tema) -> egui::Button<'static> {
    egui::Button::new(
        egui::RichText::new(texto.to_owned())
            .strong()
            .color(cor32(texto_sobre(tema.destaque))),
    )
    .fill(cor32(tema.destaque))
    .stroke(egui::Stroke::new(1.0_f32, cor32(tema.destaque_clique)))
    .rounding(egui::Rounding::same(tema.raio_widget as f32))
}
