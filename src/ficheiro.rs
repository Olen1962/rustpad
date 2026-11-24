#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::ModoTema;
use std::fs;
use std::path::PathBuf;

use crate::config::Textos;
use crate::estilo::{self, cor32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcaoFicheiro {
    Nenhuma,
    Novo,
    PedirAbrir,
    Guardar,
    PedirGuardarComo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModoDialogo {
    Abrir,
    GuardarComo,
}

pub struct EstadoFicheiro {
    pub caminho: Option<PathBuf>,
    pub modificado: bool,
    pub erro: Option<String>,
    dialogo: Option<ModoDialogo>,
    entrada_caminho: String,
}

impl Default for EstadoFicheiro {
    fn default() -> Self {
        Self {
            caminho: None,
            modificado: false,
            erro: None,
            dialogo: None,
            entrada_caminho: String::new(),
        }
    }
}

impl EstadoFicheiro {
    pub fn titulo_janela(&self, sem_nome: &str) -> String {
        let nome = self
            .caminho
            .as_ref()
            .and_then(|c| c.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| sem_nome.to_owned());
        if self.modificado {
            format!("*{nome} — RustPad")
        } else {
            format!("{nome} — RustPad")
        }
    }

    pub fn executar(&mut self, acao: AcaoFicheiro, texto: &mut String) {
        match acao {
            AcaoFicheiro::Nenhuma => {}
            AcaoFicheiro::Novo => {
                texto.clear();
                self.caminho = None;
                self.modificado = false;
                self.erro = None;
            }
            AcaoFicheiro::PedirAbrir => {
                self.entrada_caminho = self
                    .caminho
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default();
                self.dialogo = Some(ModoDialogo::Abrir);
            }
            AcaoFicheiro::Guardar => {
                if let Some(ref path) = self.caminho.clone() {
                    self.gravar(path, texto);
                } else {
                    self.dialogo = Some(ModoDialogo::GuardarComo);
                }
            }
            AcaoFicheiro::PedirGuardarComo => {
                self.entrada_caminho = self
                    .caminho
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| String::from("sem_nome.txt"));
                self.dialogo = Some(ModoDialogo::GuardarComo);
            }
        }
    }

    pub fn guardar_se_possivel(&mut self, texto: &str) {
        if let Some(ref path) = self.caminho.clone() {
            if self.modificado {
                self.gravar(path, texto);
            }
        }
    }

    pub fn desenhar_dialogos(
        &mut self,
        ctx: &egui::Context,
        texto: &mut String,
        modo_tema: ModoTema,
        t: &Textos,
    ) {
        let tema = estilo::resolver_tema(modo_tema);

        if let Some(modo) = self.dialogo {
            let titulo = match modo {
                ModoDialogo::Abrir => t.dialogo_abrir_titulo,
                ModoDialogo::GuardarComo => t.dialogo_guardar_titulo,
            };
            let mut confirmar = false;
            let mut cancelar = ctx.input(|i| i.key_pressed(egui::Key::Escape));

            egui::Window::new(titulo)
                .id(egui::Id::new("dialogo_ficheiro_rustpad"))
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .fixed_size(egui::vec2(420.0_f32, 0.0_f32))
                .frame(estilo::frame_dialogo(&tema))
                .show(ctx, |ui| {
                    ui.set_width(420.0_f32);
                    if estilo::cabecalho_dialogo(ui, estilo::IconeCabecalho::Ficheiro, titulo, &tema) {
                        cancelar = true;
                    }

                    estilo::corpo_dialogo().show(ui, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(8.0_f32, 10.0_f32);
                        estilo::cartao_grupo(&tema).show(ui, |ui| {
                            ui.set_min_width(368.0_f32);
                            estilo::titulo_seccao(ui, t.dialogo_caminho, &tema);
                            let resp = ui.add(
                                egui::TextEdit::singleline(&mut self.entrada_caminho)
                                    .desired_width(366.0_f32)
                                    .hint_text("/home/utilizador/documento.txt"),
                            );
                            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                confirmar = true;
                            }
                        });
                    });

                    let rodape = estilo::frame_rodape_dialogo(&tema).show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pode = !self.entrada_caminho.trim().is_empty();
                            if ui
                                .add_enabled(pode, estilo::botao_primario(t.confirmar, &tema))
                                .clicked()
                            {
                                confirmar = true;
                            }
                            if ui.button(t.cancelar).clicked() {
                                cancelar = true;
                            }
                        });
                    });
                    estilo::linha_separadora_topo(ui, rodape.response.rect, &tema);
                });

            if confirmar && !self.entrada_caminho.trim().is_empty() {
                let path = PathBuf::from(self.entrada_caminho.trim());
                match modo {
                    ModoDialogo::Abrir => self.ler(&path, texto),
                    ModoDialogo::GuardarComo => self.gravar(&path, texto),
                }
                self.dialogo = None;
            } else if cancelar {
                self.dialogo = None;
            }
        }

        if let Some(ref msg) = self.erro.clone() {
            let mut fechar_erro = ctx.input(|i| i.key_pressed(egui::Key::Escape));
            egui::Window::new(t.dialogo_erro_titulo)
                .id(egui::Id::new("dialogo_erro_rustpad"))
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .fixed_size(egui::vec2(380.0_f32, 0.0_f32))
                .frame(estilo::frame_dialogo(&tema))
                .show(ctx, |ui| {
                    ui.set_width(380.0_f32);
                    if estilo::cabecalho_dialogo(
                        ui,
                        estilo::IconeCabecalho::Erro,
                        t.dialogo_erro_titulo,
                        &tema,
                    ) {
                        fechar_erro = true;
                    }

                    estilo::corpo_dialogo().show(ui, |ui| {
                        estilo::cartao_grupo(&tema).show(ui, |ui| {
                            ui.set_min_width(328.0_f32);
                            ui.label(
                                egui::RichText::new(msg)
                                    .color(cor32(tema.perigo))
                                    .strong(),
                            );
                        });
                    });

                    let rodape = estilo::frame_rodape_dialogo(&tema).show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(estilo::botao_primario(t.fechar, &tema)).clicked() {
                                fechar_erro = true;
                            }
                        });
                    });
                    estilo::linha_separadora_topo(ui, rodape.response.rect, &tema);
                });
            if fechar_erro {
                self.erro = None;
            }
        }
    }

    fn ler(&mut self, path: &PathBuf, texto: &mut String) {
        match fs::read_to_string(path) {
            Ok(conteudo) => {
                *texto = conteudo;
                self.caminho = Some(path.clone());
                self.modificado = false;
                self.erro = None;
            }
            Err(e) => {
                self.erro = Some(format!("Não foi possível abrir o ficheiro: {e}"));
            }
        }
    }

    fn gravar(&mut self, path: &PathBuf, texto: &str) {
        match fs::write(path, texto) {
            Ok(()) => {
                self.caminho = Some(path.clone());
                self.modificado = false;
                self.erro = None;
            }
            Err(e) => {
                self.erro = Some(format!("Não foi possível guardar o ficheiro: {e}"));
            }
        }
    }
}
