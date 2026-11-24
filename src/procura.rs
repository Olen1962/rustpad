#![forbid(unsafe_code)]

use eframe::egui;

use crate::config::Textos;

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

    pub fn desenhar(&mut self, ctx: &egui::Context, texto: &mut String, t: &Textos) -> bool {
        if !self.aberta {
            return false;
        }

        let mut alterou_texto = false;
        let mut janela_aberta = self.aberta;
        let total = self.contar_ocorrencias(texto);
        if total == 0 {
            self.indice_atual = 0;
        } else if self.indice_atual >= total {
            self.indice_atual = total - 1;
        }

        egui::Window::new(t.procurar_substituir)
            .open(&mut janela_aberta)
            .collapsible(false)
            .resizable(true)
            .default_width(360.0)
            .show(ctx, |ui| {
                egui::Grid::new("grid_procura")
                    .num_columns(2)
                    .spacing([8.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(format!("{}:", t.procurar));
                        ui.text_edit_singleline(&mut self.termo);
                        ui.end_row();

                        ui.label(format!("{}:", t.substituir));
                        ui.text_edit_singleline(&mut self.substituir_por);
                        ui.end_row();
                    });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.distinguir_maiusculas, "Aa");
                    if total > 0 {
                        ui.label(format!("{}/{} {}", self.indice_atual + 1, total, t.resultados));
                    } else {
                        ui.label(format!("0 {}", t.resultados));
                    }
                });

                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    if ui.button(t.anterior).clicked() && total > 0 {
                        self.indice_atual = if self.indice_atual == 0 {
                            total - 1
                        } else {
                            self.indice_atual - 1
                        };
                    }
                    if ui.button(t.seguinte).clicked() && total > 0 {
                        self.indice_atual = (self.indice_atual + 1) % total;
                    }
                    if ui.button(t.substituir).clicked() && !self.termo.is_empty() && total > 0 {
                        if let Some(novo) = self.substituir_ocorrencia(texto, self.indice_atual) {
                            *texto = novo;
                            alterou_texto = true;
                        }
                    }
                    if ui.button(t.substituir_tudo).clicked() && !self.termo.is_empty() {
                        *texto = self.substituir_todas(texto);
                        self.indice_atual = 0;
                        alterou_texto = true;
                    }
                });
            });

        self.aberta = janela_aberta;
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
