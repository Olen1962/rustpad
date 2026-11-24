#![forbid(unsafe_code)]

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcaoVer {
    Nenhuma,
    ZoomMais,
    ZoomMenos,
    GuardarConfig,
}

pub fn aplicar_acao(acao: AcaoVer, config: &mut Config) -> bool {
    match acao {
        AcaoVer::Nenhuma => false,
        AcaoVer::ZoomMais => {
            config.tamanho_fonte = (config.tamanho_fonte + 1.0).min(32.0);
            let _ = config.guardar();
            true
        }
        AcaoVer::ZoomMenos => {
            config.tamanho_fonte = (config.tamanho_fonte - 1.0).max(9.0);
            let _ = config.guardar();
            true
        }
        AcaoVer::GuardarConfig => {
            let _ = config.guardar();
            false
        }
    }
}
