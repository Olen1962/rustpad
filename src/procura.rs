#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::ModoTema;

use crate::config::Textos;
use crate::estilo::{self, cor32};

#[derive(Default)]
pub struct Procura {
    pub aberta: bool,
    pub termo: String,
    pub substituir_por: String,
    pub distinguir_maiusculas: bool,
    pub indice_atual: usize,
}

impl Procura {
    pub fn contar_ocorrencias(&self, texto: &str) -> usize {
        if self.termo.is_empty() {
            return 0;
        }
        if self.distinguir_maiusculas {
            texto.matches(&self.termo).count()
        } else {
            texto
                .to_lowercase()
                .matches(&self.termo.to_lowercase())
                .count()
        }
    }

    pub fn desenhar(
        &mut self,
        ctx: &egui::Context,
        texto: &mut String,
        modo_tema: ModoTema,
        t: &Textos,
    ) -> bool {
        if !self.aberta {
            return false;
        }

        let tema = estilo::resolver_tema(modo_tema);
        let mut alterou_texto = false;
        let mut fechar = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        let total = self.contar_ocorrencias(texto);
        if total == 0 {
            self.indice_atual = 0;
        } else if self.indice_atual >= total {
            self.indice_atual = total - 1;
        }

        egui::Window::new(t.procurar_substituir)
            .id(egui::Id::new("janela_procura_rustpad"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size(egui::vec2(400.0_f32, 0.0_f32))
            .frame(estilo::frame_dialogo(&tema))
            .show(ctx, |ui| {
                ui.set_width(400.0_f32);
                if estilo::cabecalho_dialogo(
                    ui,
                    estilo::IconeCabecalho::Procura,
                    t.procurar_substituir,
                    &tema,
                ) {
                    fechar = true;
                }

                estilo::corpo_dialogo().show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0_f32, 10.0_f32);

                    estilo::cartao_grupo(&tema).show(ui, |ui| {
                        ui.set_min_width(348.0_f32);
                        egui::Grid::new("grid_procura")
                            .num_columns(2)
                            .spacing([10.0_f32, 8.0_f32])
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(t.procurar)
                                        .color(cor32(tema.texto_secundario)),
                                );
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.termo)
                                        .desired_width(256.0_f32)
                                        .hint_text(t.dica_procurar),
                                );
                                ui.end_row();

                                ui.label(
                                    egui::RichText::new(t.substituir)
                                        .color(cor32(tema.texto_secundario)),
                                );
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.substituir_por)
                                        .desired_width(256.0_f32)
                                        .hint_text(t.dica_substituir),
                                );
                                ui.end_row();
                            });
                    });

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.distinguir_maiusculas, t.maiusculas_exatas);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let etiqueta = if total > 0 {
                                format!("{}/{} {}", self.indice_atual + 1, total, t.resultados)
                            } else {
                                format!("0 {}", t.resultados)
                            };
                            let cor_cont = if total > 0 {
                                cor32(tema.destaque)
                            } else {
                                cor32(tema.texto_secundario)
                            };
                            ui.label(egui::RichText::new(etiqueta).small().strong().color(cor_cont));
                        });
                    });

                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(total > 0, egui::Button::new(format!("◀ {}", t.anterior)))
                            .clicked()
                        {
                            self.indice_atual = if self.indice_atual == 0 {
                                total - 1
                            } else {
                                self.indice_atual - 1
                            };
                        }
                        if ui
                            .add_enabled(total > 0, egui::Button::new(format!("{} ▶", t.seguinte)))
                            .clicked()
                        {
                            self.indice_atual = (self.indice_atual + 1) % total;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pode = !self.termo.is_empty() && total > 0;
                            if ui
                                .add_enabled(pode, estilo::botao_primario(t.substituir_tudo, &tema))
                                .clicked()
                            {
                                *texto = self.substituir_todas(texto);
                                self.indice_atual = 0;
                                alterou_texto = true;
                            }
                            if ui
                                .add_enabled(pode, egui::Button::new(t.substituir))
                                .clicked()
                            {
                                if let Some(novo) = self.substituir_ocorrencia(texto, self.indice_atual)
                                {
                                    *texto = novo;
                                    alterou_texto = true;
                                }
                            }
                        });
                    });
                });
            });

        if fechar {
            self.aberta = false;
        }
        alterou_texto
    }

    fn substituir_ocorrencia(&self, texto: &str, alvo_idx: usize) -> Option<String> {
        let posicoes = self.encontrar_intervalos(texto);
        let (inicio, fim) = *posicoes.get(alvo_idx)?;
        let mut out = String::with_capacity(texto.len() + self.substituir_por.len());
        out.push_str(&texto[..inicio]);
        out.push_str(&self.substituir_por);
        out.push_str(&texto[fim..]);
        Some(out)
    }

    fn substituir_todas(&self, texto: &str) -> String {
        if self.distinguir_maiusculas {
            return texto.replace(&self.termo, &self.substituir_por);
        }
        let intervalos = self.encontrar_intervalos(texto);
        if intervalos.is_empty() {
            return texto.to_owned();
        }
        let mut out = String::with_capacity(texto.len());
        let mut cursor = 0;
        for (inicio, fim) in intervalos {
            out.push_str(&texto[cursor..inicio]);
            out.push_str(&self.substituir_por);
            cursor = fim;
        }
        out.push_str(&texto[cursor..]);
        out
    }

    fn encontrar_intervalos(&self, texto: &str) -> Vec<(usize, usize)> {
        if self.termo.is_empty() {
            return Vec::new();
        }
        if self.distinguir_maiusculas {
            return texto
                .match_indices(&self.termo)
                .map(|(idx, s)| (idx, idx + s.len()))
                .collect();
        }
        let agulha = self.termo.to_lowercase();
        let palheiro = texto.to_lowercase();
        let mut out = Vec::new();
        let mut pos = 0;
        while let Some(rel) = palheiro[pos..].find(&agulha) {
            let inicio = pos + rel;
            let fim = inicio + agulha.len();
            if texto.is_char_boundary(inicio) && texto.is_char_boundary(fim) {
                out.push((inicio, fim));
            }
            pos = fim;
        }
        out
    }
}
