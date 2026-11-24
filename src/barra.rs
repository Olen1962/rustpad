#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::{Tema, texto_sobre};

use crate::config::{Config, Textos};
use crate::editar::AcaoEditar;
use crate::estilo::cor32;
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

pub fn desenhar(
    ctx: &egui::Context,
    titulo: &str,
    config: &mut Config,
) -> ResultadoBarra {
    let mut res = ResultadoBarra::default();
    let tema = Tema::obter(config.modo_tema);
    let t = Textos::de(config.idioma);

    let frame = egui::Frame {
        inner_margin: egui::Margin::symmetric(10.0, 6.0),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::ZERO,
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.barra_titulo),
        stroke: egui::Stroke::new(1.0, cor32(tema.separador)),
    };

    egui::TopBottomPanel::top("barra_gtk4")
        .frame(frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
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

                let area_arrasto = ui.available_rect_before_wrap();
                let resp = ui.interact(
                    egui::Rect::from_min_max(
                        area_arrasto.min,
                        egui::pos2(
                            (area_arrasto.max.x - 108.0).max(area_arrasto.min.x),
                            area_arrasto.max.y,
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

                ui.label(
                    egui::RichText::new(titulo)
                        .strong()
                        .color(cor32(tema.texto_principal)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let fechar = egui::Button::new(
                        egui::RichText::new("✕").color(cor32(texto_sobre(tema.perigo))),
                    )
                    .fill(cor32(tema.perigo));
                    if ui.add(fechar).clicked() {
                        res.sair = true;
                    }
                    if ui.button("□").clicked() {
                        let max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
                    }
                    if ui.button("—").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    }
                });
            });
        });

    desenhar_toolbar(ctx, config, &mut res);
    res
}

fn desenhar_toolbar(ctx: &egui::Context, config: &mut Config, res: &mut ResultadoBarra) {
    let tema = Tema::obter(config.modo_tema);
    let t = Textos::de(config.idioma);

    let frame = egui::Frame {
        inner_margin: egui::Margin::symmetric(10.0, 5.0),
        outer_margin: egui::Margin::ZERO,
        rounding: egui::Rounding::ZERO,
        shadow: egui::epaint::Shadow::NONE,
        fill: cor32(tema.fundo_cartao),
        stroke: egui::Stroke::new(1.0, cor32(tema.separador)),
    };

    egui::TopBottomPanel::top("toolbar_rustpad")
        .frame(frame)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("📄").on_hover_text(t.novo).clicked() {
                    res.ficheiro = AcaoFicheiro::Novo;
                }
                if ui.button("📂").on_hover_text(t.abrir).clicked() {
                    res.ficheiro = AcaoFicheiro::PedirAbrir;
                }
                if ui.button("💾").on_hover_text(t.guardar).clicked() {
                    res.ficheiro = AcaoFicheiro::Guardar;
                }
                if ui.button("📑").on_hover_text(t.guardar_como).clicked() {
                    res.ficheiro = AcaoFicheiro::PedirGuardarComo;
                }
                if ui.button("✖").on_hover_text(t.fechar).clicked() {
                    res.fechar_documento = true;
                }
                ui.separator();
                if ui.button("↶").on_hover_text(t.desfazer).clicked() {
                    res.editar = AcaoEditar::Desfazer;
                }
                if ui.button("↷").on_hover_text(t.refazer).clicked() {
                    res.editar = AcaoEditar::Refazer;
                }
                if ui.button("🔍").on_hover_text(t.procurar_substituir).clicked() {
                    res.alternar_procura = true;
                }
                ui.separator();
                if ui.selectable_label(config.numeros_linha, "#").on_hover_text(t.numeros_linha).clicked() {
                    config.numeros_linha = !config.numeros_linha;
                    res.ver = AcaoVer::GuardarConfig;
                }
                if ui.selectable_label(config.quebra_linha, "↩").on_hover_text(t.quebra_linha).clicked() {
                    config.quebra_linha = !config.quebra_linha;
                    res.ver = AcaoVer::GuardarConfig;
                }
                if ui.selectable_label(config.barra_estado, "▬").on_hover_text(t.barra_estado).clicked() {
                    config.barra_estado = !config.barra_estado;
                    res.ver = AcaoVer::GuardarConfig;
                }
                if ui.button("A+").on_hover_text(t.zoom_mais).clicked() {
                    res.ver = AcaoVer::ZoomMais;
                }
                if ui.button("A-").on_hover_text(t.zoom_menos).clicked() {
                    res.ver = AcaoVer::ZoomMenos;
                }
                ui.separator();
                if ui.button("⚙").on_hover_text(t.preferencias).clicked() {
                    res.abrir_preferencias = true;
                }
            });
        });
}
