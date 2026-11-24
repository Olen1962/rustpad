#![forbid(unsafe_code)]

use eframe::egui;
use std::fs;
use std::path::PathBuf;

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

    pub fn desenhar_dialogos(&mut self, ctx: &egui::Context, texto: &mut String) {
        if let Some(modo) = self.dialogo {
            let titulo = match modo {
                ModoDialogo::Abrir => "Abrir ficheiro",
                ModoDialogo::GuardarComo => "Guardar ficheiro como",
            };
            let mut aberto = true;
            let mut confirmar = false;
            let mut cancelar = false;

            egui::Window::new(titulo)
                .collapsible(false)
                .resizable(false)
                .open(&mut aberto)
                .show(ctx, |ui| {
                    ui.label("Caminho do ficheiro:");
                    let resp = ui.text_edit_singleline(&mut self.entrada_caminho);
                    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        confirmar = true;
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Confirmar").clicked() {
                            confirmar = true;
                        }
                        if ui.button("Cancelar").clicked() {
                            cancelar = true;
                        }
                    });
                });

            if confirmar && !self.entrada_caminho.trim().is_empty() {
                let path = PathBuf::from(self.entrada_caminho.trim());
                match modo {
                    ModoDialogo::Abrir => self.ler(&path, texto),
                    ModoDialogo::GuardarComo => self.gravar(&path, texto),
                }
                self.dialogo = None;
            } else if cancelar || !aberto {
                self.dialogo = None;
            }
        }

        if let Some(ref msg) = self.erro.clone() {
            let mut aberto = true;
            egui::Window::new("Erro de ficheiro")
                .collapsible(false)
                .resizable(false)
                .open(&mut aberto)
                .show(ctx, |ui| {
                    ui.colored_label(ui.visuals().error_fg_color, msg);
                    if ui.button("Fechar").clicked() {
                        self.erro = None;
                    }
                });
            if !aberto {
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
