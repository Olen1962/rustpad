#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::ModoTema;

use crate::config::{Config, Idioma, Textos};
use crate::estilo::{self, cor32};
use crate::fontes;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AbaPreferencias {
    #[default]
    Aparencia,
    Editor,
    Sistema,
}

pub struct JanelaPreferencias {
    pub aberta: bool,
    aba: AbaPreferencias,
    fontes_disponiveis: Vec<String>,
}

impl Default for JanelaPreferencias {
    fn default() -> Self {
        Self {
            aberta: false,
            aba: AbaPreferencias::Aparencia,
            fontes_disponiveis: fontes::listar_fontes_sistema(),
        }
    }
}

impl JanelaPreferencias {
    pub fn desenhar(&mut self, ctx: &egui::Context, config: &mut Config) -> bool {
        if !self.aberta {
            return false;
        }

        const LARGURA_JANELA: f32 = 440.0_f32;
        const LARGURA_CONTEUDO: f32 = 412.0_f32;
        const LARGURA_CARTAO: f32 = 386.0_f32;

        let t = Textos::de(config.idioma);
        let tema = estilo::resolver_tema(config.modo_tema);
        let mut fechar_janela = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        let mut tema_alterado = false;
        let mut fonte_alterada = false;
        let mut config_alterada = false;
        let mut icones_alterados = false;

        egui::Window::new(t.prefs_titulo)
            .id(egui::Id::new("janela_preferencias_rustpad"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size(egui::vec2(LARGURA_JANELA, 0.0_f32))
            .frame(estilo::frame_dialogo(&tema))
            .show(ctx, |ui| {
                ui.set_width(LARGURA_JANELA);

                if estilo::cabecalho_dialogo(
                    ui,
                    estilo::IconeCabecalho::Preferencias,
                    t.prefs_titulo,
                    &tema,
                ) {
                    fechar_janela = true;
                }

                // Barra de abas distribuída por toda a largura
                let barra_abas = egui::Frame {
                    inner_margin: egui::Margin::symmetric(14.0_f32, 9.0_f32),
                    outer_margin: egui::Margin::ZERO,
                    rounding: egui::Rounding::ZERO,
                    shadow: egui::epaint::Shadow::NONE,
                    fill: cor32(tema.fundo_janela),
                    stroke: egui::Stroke::NONE,
                }
                .show(ui, |ui| {
                    let largura_aba = (LARGURA_CONTEUDO - 12.0_f32) / 3.0_f32;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0_f32;
                        for (aba, rotulo) in [
                            (AbaPreferencias::Aparencia, t.aba_aparencia),
                            (AbaPreferencias::Editor, t.aba_editor),
                            (AbaPreferencias::Sistema, t.aba_sistema),
                        ] {
                            if ui
                                .add_sized(
                                    [largura_aba, 28.0_f32],
                                    estilo::botao_segmento(rotulo, self.aba == aba, &tema),
                                )
                                .clicked()
                            {
                                self.aba = aba;
                            }
                        }
                    });
                });
                estilo::linha_separadora_base(ui, barra_abas.response.rect, &tema);

                // Corpo com cartões agrupados e largura alinhada
                estilo::corpo_dialogo().show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0_f32, 10.0_f32);
                    ui.set_width(LARGURA_CONTEUDO);

                    match self.aba {
                        AbaPreferencias::Aparencia => {
                            estilo::titulo_seccao(ui, t.prefs_tema, &tema);
                            estilo::cartao_grupo(&tema).show(ui, |ui| {
                                ui.set_width(LARGURA_CARTAO);
                                let largura_btn = (LARGURA_CARTAO - 12.0_f32) / 3.0_f32;

                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 6.0_f32;
                                    for (modo, rotulo) in [
                                        (ModoTema::Claro, t.tema_claro),
                                        (ModoTema::Escuro, t.tema_escuro),
                                        (ModoTema::Sistema, t.tema_sistema),
                                    ] {
                                        if ui
                                            .add_sized(
                                                [largura_btn, 28.0_f32],
                                                estilo::botao_segmento(
                                                    rotulo,
                                                    config.modo_tema == modo,
                                                    &tema,
                                                ),
                                            )
                                            .clicked()
                                        {
                                            config.modo_tema = modo;
                                            tema_alterado = true;
                                        }
                                    }
                                });

                                ui.separator();

                                ui.label(t.prefs_idioma);
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 6.0_f32;
                                    for idioma in [Idioma::Pt, Idioma::En, Idioma::Es] {
                                        if ui
                                            .add_sized(
                                                [largura_btn, 26.0_f32],
                                                estilo::botao_segmento(
                                                    idioma.nome(),
                                                    config.idioma == idioma,
                                                    &tema,
                                                ),
                                            )
                                            .clicked()
                                        {
                                            config.idioma = idioma;
                                            config_alterada = true;
                                        }
                                    }
                                });
                            });

                            estilo::titulo_seccao(ui, t.prefs_tipografia, &tema);
                            estilo::cartao_grupo(&tema).show(ui, |ui| {
                                ui.set_width(LARGURA_CARTAO);

                                ui.allocate_ui_with_layout(
                                    egui::vec2(LARGURA_CARTAO, 26.0_f32),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.set_min_width(LARGURA_CARTAO);
                                        ui.label(t.prefs_familia);
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                egui::ComboBox::from_id_salt("combo_familia_fonte")
                                                    .width(190.0_f32)
                                                    .selected_text(&config.fonte_familia)
                                                    .show_ui(ui, |ui| {
                                                        for nome in &self.fontes_disponiveis {
                                                            if ui
                                                                .selectable_value(
                                                                    &mut config.fonte_familia,
                                                                    nome.clone(),
                                                                    nome,
                                                                )
                                                                .changed()
                                                            {
                                                                fonte_alterada = true;
                                                            }
                                                        }
                                                    });
                                            },
                                        );
                                    },
                                );

                                // Tamanho do editor (título + valor alinhados e slider em toda a largura)
                                ui.allocate_ui_with_layout(
                                    egui::vec2(LARGURA_CARTAO, 18.0_f32),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.set_min_width(LARGURA_CARTAO);
                                        ui.label(t.prefs_tam_editor);
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(format!(
                                                        "{:.0} pt",
                                                        config.tamanho_fonte
                                                    ))
                                                    .monospace()
                                                    .strong()
                                                    .color(cor32(tema.destaque)),
                                                );
                                            },
                                        );
                                    },
                                );
                                {
                                    let antigo = ui.spacing().slider_width;
                                    ui.spacing_mut().slider_width = LARGURA_CARTAO - 8.0_f32;
                                    if ui
                                        .add(
                                            egui::Slider::new(
                                                &mut config.tamanho_fonte,
                                                9.0_f32..=32.0_f32,
                                            )
                                            .step_by(1.0)
                                            .show_value(false),
                                        )
                                        .changed()
                                    {
                                        fonte_alterada = true;
                                    }
                                    ui.spacing_mut().slider_width = antigo;
                                }

                                // Tamanho dos menus (título + valor alinhados e slider em toda a largura)
                                ui.allocate_ui_with_layout(
                                    egui::vec2(LARGURA_CARTAO, 18.0_f32),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.set_min_width(LARGURA_CARTAO);
                                        ui.label(t.prefs_tam_menu);
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(format!(
                                                        "{:.0} pt",
                                                        config.tamanho_menu
                                                    ))
                                                    .monospace()
                                                    .strong()
                                                    .color(cor32(tema.destaque)),
                                                );
                                            },
                                        );
                                    },
                                );
                                {
                                    let antigo = ui.spacing().slider_width;
                                    ui.spacing_mut().slider_width = LARGURA_CARTAO - 8.0_f32;
                                    if ui
                                        .add(
                                            egui::Slider::new(
                                                &mut config.tamanho_menu,
                                                10.0_f32..=26.0_f32,
                                            )
                                            .step_by(1.0)
                                            .show_value(false),
                                        )
                                        .changed()
                                    {
                                        fonte_alterada = true;
                                    }
                                    ui.spacing_mut().slider_width = antigo;
                                }
                            });
                        }

                        AbaPreferencias::Editor => {
                            estilo::titulo_seccao(ui, t.prefs_ajustes, &tema);
                            estilo::cartao_grupo(&tema).show(ui, |ui| {
                                ui.set_width(LARGURA_CARTAO);
                                ui.allocate_ui_with_layout(
                                    egui::vec2(LARGURA_CARTAO, 26.0_f32),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.set_min_width(LARGURA_CARTAO);
                                        ui.label(t.prefs_largura_tab);
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.spacing_mut().item_spacing.x = 4.0_f32;
                                                for w in [8_u8, 4, 2] {
                                                    if ui
                                                        .add_sized(
                                                            [34.0_f32, 24.0_f32],
                                                            estilo::botao_segmento(
                                                                &format!("{w}"),
                                                                config.largura_tab == w,
                                                                &tema,
                                                            ),
                                                        )
                                                        .clicked()
                                                    {
                                                        config.largura_tab = w;
                                                        config_alterada = true;
                                                    }
                                                }
                                            },
                                        );
                                    },
                                );

                                ui.separator();

                                if estilo::linha_checkbox(
                                    ui,
                                    t.prefs_espacos_tab,
                                    &mut config.usar_espacos_tab,
                                    LARGURA_CARTAO,
                                ) {
                                    config_alterada = true;
                                }
                                if estilo::linha_checkbox(
                                    ui,
                                    t.prefs_auto_indentar,
                                    &mut config.auto_indentar,
                                    LARGURA_CARTAO,
                                ) {
                                    config_alterada = true;
                                }
                                if estilo::linha_checkbox(
                                    ui,
                                    t.prefs_destacar_linha,
                                    &mut config.destacar_linha_atual,
                                    LARGURA_CARTAO,
                                ) {
                                    config_alterada = true;
                                }
                                if estilo::linha_checkbox(
                                    ui,
                                    t.prefs_guardar_auto,
                                    &mut config.guardar_automatico,
                                    LARGURA_CARTAO,
                                ) {
                                    config_alterada = true;
                                }
                            });
                        }

                        AbaPreferencias::Sistema => {
                            estilo::titulo_seccao(ui, t.prefs_icones, &tema);
                            estilo::cartao_grupo(&tema).show(ui, |ui| {
                                ui.set_width(LARGURA_CARTAO);
                                ui.label(t.prefs_pasta_icones);
                                if ui
                                    .add(
                                        egui::TextEdit::singleline(&mut config.pasta_icones)
                                            .desired_width(LARGURA_CARTAO),
                                    )
                                    .changed()
                                {
                                    icones_alterados = true;
                                    config_alterada = true;
                                }

                                let largura_btn = (LARGURA_CARTAO - 12.0_f32) / 3.0_f32;
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 6.0_f32;
                                    if ui
                                        .add_sized([largura_btn, 26.0_f32], egui::Button::new("GartoonRedux"))
                                        .clicked()
                                    {
                                        config.pasta_icones = String::from(
                                            "/usr/local/share/icons/GartoonRedux/24x24/actions",
                                        );
                                        icones_alterados = true;
                                        config_alterada = true;
                                    }
                                    if ui
                                        .add_sized([largura_btn, 26.0_f32], egui::Button::new("Adwaita"))
                                        .clicked()
                                    {
                                        let local = std::path::Path::new("/usr/local/share/icons/Adwaita");
                                        config.pasta_icones = if local.is_dir() {
                                            String::from("/usr/local/share/icons/Adwaita")
                                        } else {
                                            String::from("/usr/share/icons/Adwaita")
                                        };
                                        icones_alterados = true;
                                        config_alterada = true;
                                    }
                                    if ui
                                        .add_sized(
                                            [largura_btn, 26.0_f32],
                                            egui::Button::new(t.prefs_detetar_xfce),
                                        )
                                        .clicked()
                                    {
                                        if let Some(pasta) = estilo::detetar_pasta_icones_xfce() {
                                            config.pasta_icones = pasta;
                                            icones_alterados = true;
                                            config_alterada = true;
                                        }
                                        if let Some(tam) = estilo::detetar_tamanho_menu_xfce() {
                                            config.tamanho_menu = tam;
                                            fonte_alterada = true;
                                        }
                                    }
                                });

                                ui.separator();

                                if estilo::linha_checkbox(
                                    ui,
                                    t.prefs_decoracoes_sistema,
                                    &mut config.decoracoes_sistema,
                                    LARGURA_CARTAO,
                                ) {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(
                                        config.decoracoes_sistema,
                                    ));
                                    config_alterada = true;
                                }
                            });
                        }
                    }
                });

                // Rodapé limpo igual ao preview
                let rodape = estilo::frame_rodape_dialogo(&tema).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(estilo::botao_primario(t.fechar, &tema)).clicked() {
                            let _ = config.guardar();
                            fechar_janela = true;
                        }
                    });
                });
                estilo::linha_separadora_topo(ui, rodape.response.rect, &tema);
            });

        if fechar_janela {
            self.aberta = false;
        }

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
        icones_alterados
    }
}
