#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcaoEditar {
    Nenhuma,
    Desfazer,
    Refazer,
}

pub struct Historico {
    desfazer: Vec<String>,
    refazer: Vec<String>,
    ultimo_registado: String,
}

impl Default for Historico {
    fn default() -> Self {
        Self {
            desfazer: Vec::new(),
            refazer: Vec::new(),
            ultimo_registado: String::new(),
        }
    }
}

impl Historico {
    pub fn registar(&mut self, texto_atual: &str) {
        if texto_atual != self.ultimo_registado {
            self.desfazer.push(self.ultimo_registado.clone());
            if self.desfazer.len() > 200 {
                self.desfazer.remove(0);
            }
            self.ultimo_registado = texto_atual.to_owned();
            self.refazer.clear();
        }
    }

    pub fn limpar(&mut self, base: &str) {
        self.desfazer.clear();
        self.refazer.clear();
        self.ultimo_registado = base.to_owned();
    }

    pub fn executar(&mut self, acao: AcaoEditar, texto: &mut String) -> bool {
        match acao {
            AcaoEditar::Nenhuma => false,
            AcaoEditar::Desfazer => {
                if let Some(anterior) = self.desfazer.pop() {
                    self.refazer.push(texto.clone());
                    *texto = anterior.clone();
                    self.ultimo_registado = anterior;
                    return true;
                }
                false
            }
            AcaoEditar::Refazer => {
                if let Some(proximo) = self.refazer.pop() {
                    self.desfazer.push(texto.clone());
                    *texto = proximo.clone();
                    self.ultimo_registado = proximo;
                    return true;
                }
                false
            }
        }
    }
}
