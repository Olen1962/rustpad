#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::Tema;

use crate::barra;
use crate::config::{Config, Textos};
use crate::editar::{AcaoEditar, Historico};
use crate::estilo::{self, cor32};
use crate::ficheiro::{AcaoFicheiro, EstadoFicheiro};
use crate::preferencias::JanelaPreferencias;
use crate::procura::Procura;
use crate::ver;

pub struct Editor {
    config: Config,
    texto: String,
    ficheiro: EstadoFicheiro,
    historico: Historico,
    procura: Procura,
    preferencias: JanelaPreferencias,
}

impl Editor {
    pub fn novo(config: Config) -> Self {
        Self {
            config,
            texto: String::new(),
            ficheiro: EstadoFicheiro::default(),
            historico: Historico::default(),
            procura: Procura::default(),
            preferencias: JanelaPreferencias::default(),
        }
    }

    fn processar_atalhos(&mut self, ctx: &egui::Context) {
        let (novo, abrir, guardar, procura, prefs, desfazer, refazer, sair) = ctx.input(|i| {
            let cmd = i.modifiers.command || i.modifiers.ctrl;
            (
                cmd && i.key_pressed(egui::Key::N),
                cmd && i.key_pressed(egui::Key::O),
                cmd && i.key_pressed(egui::Key::S),
                cmd && i.key_pressed(egui::Key::F),
                cmd && i.key_pressed(egui::Key::Comma),
                cmd && i.key_pressed(egui::Key::Z),
                cmd && i.key_pressed(egui::Key::Y),
                cmd && i.key_pressed(egui::Key::Q),
            )
        });

        if novo {
            self.ficheiro.executar(AcaoFicheiro::Novo, &mut self.texto);
            self.historico.limpar(&self.texto);
        }
        if abrir {
            self.ficheiro.executar(AcaoFicheiro::PedirAbrir, &mut self.texto);
        }
        if guardar {
            self.ficheiro.executar(AcaoFicheiro::Guardar, &mut self.texto);
        }
        if procura {
            self.procura.aberta = !self.procura.aberta;
        }
        if prefs {
            self.preferencias.aberta = !self.preferencias.aberta;
        }
        if desfazer {
            self.historico.executar(AcaoEditar::Desfazer, &mut self.texto);
        }
        if refazer {
            self.historico.executar(AcaoEditar::Refazer, &mut self.texto);
        }
        if sair {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.processar_atalhos(ctx);

        let t = Textos::de(self.config.idioma);
        let titulo = self.ficheiro.titulo_janela(t.sem_nome);
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(titulo.clone()));

        let res = barra::desenhar(ctx, &titulo, &mut self.config);
        if res.sair {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if res.fechar_documento {
            self.ficheiro.executar(AcaoFicheiro::Novo, &mut self.texto);
            self.historico.limpar(&self.texto);
        }
        if res.alternar_procura {
            self.procura.aberta = !self.procura.aberta;
        }
        if res.abrir_preferencias {
            self.preferencias.aberta = true;
        }

        self.ficheiro.executar(res.ficheiro, &mut self.texto);
        if res.ficheiro == AcaoFicheiro::Novo {
            self.historico.limpar(&self.texto);
        }

        if self.historico.executar(res.editar, &mut self.texto) {
            self.ficheiro.modificado = true;
        }

        if ver::aplicar_acao(res.ver, &mut self.config) {
            estilo::aplicar_fontes(ctx, &self.config);
        }

        let tema = Tema::obter(self.config.modo_tema);

        if self.config.barra_estado {
            egui::TopBottomPanel::bottom("barra_estado")
                .frame(egui::Frame {
                    inner_margin: egui::Margin::symmetric(12.0, 5.0),
                    outer_margin: egui::Margin::ZERO,
                    rounding: egui::Rounding::ZERO,
                    shadow: egui::epaint::Shadow::NONE,
                    fill: cor32(tema.barra_titulo),
                    stroke: egui::Stroke::new(1.0, cor32(tema.separador)),
                })
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        let linhas = self.texto.lines().count().max(1);
                        ui.label(
                            egui::RichText::new(format!(
                                "{linhas} · {} car. · Tab {}",
                                self.texto.chars().count(),
                                self.config.largura_tab
                            ))
                            .small()
                            .color(cor32(tema.texto_secundario)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} · {:?}",
                                    self.config.idioma.nome(),
                                    tema.modo
                                ))
                                .small()
                                .color(cor32(tema.destaque)),
                            );
                        });
                    });
                });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::same(0.0),
                outer_margin: egui::Margin::ZERO,
                rounding: egui::Rounding::ZERO,
                shadow: egui::epaint::Shadow::NONE,
                fill: cor32(tema.fundo_texto),
                stroke: egui::Stroke::NONE,
            })
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        if self.config.numeros_linha {
                            let total = self.texto.split('\n').count().max(1);
                            let mut coluna = String::new();
                            for idx in 1..=total {
                                coluna.push_str(&format!("{idx:>4}\n"));
                            }
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(coluna)
                                        .monospace()
                                        .color(cor32(tema.texto_desativado)),
                                )
                                .selectable(false),
                            );
                        }

                        let resp = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut self.texto)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .frame(false),
                        );

                        if resp.changed() {
                            self.ficheiro.modificado = true;
                            self.historico.registar(&self.texto);
                            if self.config.guardar_automatico {
                                self.ficheiro.guardar_se_possivel(&self.texto);
                            }
                        }
                    });
                });
            });

        if self.procura.desenhar(ctx, &mut self.texto, &t) {
            self.ficheiro.modificado = true;
            self.historico.registar(&self.texto);
        }

        self.ficheiro.desenhar_dialogos(ctx, &mut self.texto);
        self.preferencias.desenhar(ctx, &mut self.config);
    }
}
