#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::ModoTema;

use crate::config::{Config, Idioma, Textos};
use crate::estilo;
use crate::fontes;

pub struct JanelaPreferencias {
    pub aberta: bool,
    fontes_disponiveis: Vec<String>,
}

impl Default for JanelaPreferencias {
    fn default() -> Self {
        Self {
            aberta: false,
            fontes_disponiveis: fontes::listar_fontes_sistema(),
        }
    }
}

impl JanelaPreferencias {
    pub fn desenhar(&mut self, ctx: &egui::Context, config: &mut Config) {
        if !self.aberta {
            return;
        }

        let t = Textos::de(config.idioma);
        let mut aberta = self.aberta;
        let mut tema_alterado = false;
        let mut fonte_alterada = false;
        let mut config_alterada = false;

        egui::Window::new(t.prefs_titulo)
            .open(&mut aberta)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading(t.prefs_idioma);
                ui.horizontal(|ui| {
                    for idioma in [Idioma::Pt, Idioma::En, Idioma::Es] {
                        if ui
                            .selectable_value(&mut config.idioma, idioma, idioma.nome())
                            .changed()
                        {
                            config_alterada = true;
                        }
                    }
                });

                ui.separator();
                ui.heading(t.prefs_tema);
                ui.horizontal(|ui| {
                    if ui
                        .selectable_value(&mut config.modo_tema, ModoTema::Claro, "Claro")
                        .changed()
                    {
                        tema_alterado = true;
                    }
                    if ui
                        .selectable_value(&mut config.modo_tema, ModoTema::Escuro, "Escuro")
                        .changed()
                    {
                        tema_alterado = true;
                    }
                    if ui
                        .selectable_value(&mut config.modo_tema, ModoTema::Classico, "Clássico")
                        .changed()
                    {
                        tema_alterado = true;
                    }
                });

                ui.separator();
                ui.heading(t.prefs_tipografia);

                egui::ComboBox::from_label("Família")
                    .selected_text(&config.fonte_familia)
                    .show_ui(ui, |ui| {
                        for nome in &self.fontes_disponiveis {
                            if ui
                                .selectable_value(&mut config.fonte_familia, nome.clone(), nome)
                                .changed()
                            {
                                fonte_alterada = true;
                            }
                        }
                    });

                if ui
                    .add(egui::Slider::new(&mut config.tamanho_fonte, 9.0..=32.0).text("Editor (pt)"))
                    .changed()
                {
                    fonte_alterada = true;
                }

                if ui
                    .add(egui::Slider::new(&mut config.tamanho_menu, 10.0..=26.0).text("Menus (pt)"))
                    .changed()
                {
                    fonte_alterada = true;
                }

                ui.separator();
                ui.heading(t.prefs_ajustes);
                ui.horizontal(|ui| {
                    ui.label(t.prefs_largura_tab);
                    for w in [2_u8, 4, 8] {
                        if ui
                            .selectable_value(&mut config.largura_tab, w, format!("{w}"))
                            .changed()
                        {
                            config_alterada = true;
                        }
                    }
                });
                if ui.checkbox(&mut config.usar_espacos_tab, t.prefs_espacos_tab).changed() {
                    config_alterada = true;
                }
                if ui.checkbox(&mut config.auto_indentar, t.prefs_auto_indentar).changed() {
                    config_alterada = true;
                }
                if ui.checkbox(&mut config.destacar_linha_atual, t.prefs_destacar_linha).changed() {
                    config_alterada = true;
                }
                if ui.checkbox(&mut config.guardar_automatico, t.prefs_guardar_auto).changed() {
                    config_alterada = true;
                }

                ui.separator();
                if ui.button(t.guardar_defs).clicked() {
                    let _ = config.guardar();
                }
            });

        self.aberta = aberta;

        if tema_alterado {
            estilo::aplicar_tema(ctx, config);
            let _ = config.guardar();
        }
        if fonte_alterada {
            estilo::aplicar_fontes(ctx, config);
            let _ = config.guardar();
        }
        if config_alterada {
            let _ = config.guardar();
        }
    }
}
