#![forbid(unsafe_code)]

use rustpad::ModoTema;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Idioma {
    Pt,
    En,
    Es,
}

impl Idioma {
    pub fn nome(self) -> &'static str {
        match self {
            Self::Pt => "Português",
            Self::En => "English",
            Self::Es => "Español",
        }
    }

    pub fn codigo(self) -> &'static str {
        match self {
            Self::Pt => "pt",
            Self::En => "en",
            Self::Es => "es",
        }
    }

    pub fn de_codigo(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "en" => Self::En,
            "es" => Self::Es,
            _ => Self::Pt,
        }
    }
}

pub struct Textos {
    pub sem_nome: &'static str,
    pub menu_ficheiro: &'static str,
    pub menu_editar: &'static str,
    pub menu_ver: &'static str,
    pub novo: &'static str,
    pub abrir: &'static str,
    pub guardar: &'static str,
    pub guardar_como: &'static str,
    pub fechar: &'static str,
    pub sair: &'static str,
    pub desfazer: &'static str,
    pub refazer: &'static str,
    pub procurar: &'static str,
    pub substituir: &'static str,
    pub substituir_tudo: &'static str,
    pub anterior: &'static str,
    pub seguinte: &'static str,
    pub resultados: &'static str,
    pub procurar_substituir: &'static str,
    pub preferencias: &'static str,
    pub numeros_linha: &'static str,
    pub quebra_linha: &'static str,
    pub barra_estado: &'static str,
    pub zoom_mais: &'static str,
    pub zoom_menos: &'static str,
    pub prefs_titulo: &'static str,
    pub prefs_idioma: &'static str,
    pub prefs_tema: &'static str,
    pub prefs_tipografia: &'static str,
    pub prefs_ajustes: &'static str,
    pub prefs_largura_tab: &'static str,
    pub prefs_espacos_tab: &'static str,
    pub prefs_auto_indentar: &'static str,
    pub prefs_destacar_linha: &'static str,
    pub prefs_guardar_auto: &'static str,
    pub guardar_defs: &'static str,
}

impl Textos {
    pub const fn de(idioma: Idioma) -> Self {
        match idioma {
            Idioma::Pt => Self {
                sem_nome: "Sem nome",
                menu_ficheiro: "Ficheiro",
                menu_editar: "Editar",
                menu_ver: "Ver",
                novo: "Novo",
                abrir: "Abrir…",
                guardar: "Guardar",
                guardar_como: "Guardar como…",
                fechar: "Fechar",
                sair: "Sair",
                desfazer: "Desfazer",
                refazer: "Refazer",
                procurar: "Procurar",
                substituir: "Substituir",
                substituir_tudo: "Substituir tudo",
                anterior: "Anterior",
                seguinte: "Seguinte",
                resultados: "resultados",
                procurar_substituir: "Procurar / Substituir",
                preferencias: "Preferências…",
                numeros_linha: "Números de linha",
                quebra_linha: "Quebra de linha",
                barra_estado: "Barra de estado",
                zoom_mais: "Aumentar letra",
                zoom_menos: "Diminuir letra",
                prefs_titulo: "Preferências do RustPad",
                prefs_idioma: "Idioma (i18n)",
                prefs_tema: "Tema GTK4 (src/lib.rs)",
                prefs_tipografia: "Tipografia",
                prefs_ajustes: "Ajustes do Editor",
                prefs_largura_tab: "Largura do Tab",
                prefs_espacos_tab: "Inserir espaços em vez de Tab",
                prefs_auto_indentar: "Auto-indentação",
                prefs_destacar_linha: "Destacar linha atual",
                prefs_guardar_auto: "Guardar automaticamente",
                guardar_defs: "Guardar definições",
            },
            Idioma::En => Self {
                sem_nome: "Untitled",
                menu_ficheiro: "File",
                menu_editar: "Edit",
                menu_ver: "View",
                novo: "New",
                abrir: "Open…",
                guardar: "Save",
                guardar_como: "Save as…",
                fechar: "Close",
                sair: "Quit",
                desfazer: "Undo",
                refazer: "Redo",
                procurar: "Find",
                substituir: "Replace",
                substituir_tudo: "Replace all",
                anterior: "Previous",
                seguinte: "Next",
                resultados: "matches",
                procurar_substituir: "Find / Replace",
                preferencias: "Preferences…",
                numeros_linha: "Line numbers",
                quebra_linha: "Word wrap",
                barra_estado: "Status bar",
                zoom_mais: "Zoom in",
                zoom_menos: "Zoom out",
                prefs_titulo: "RustPad Preferences",
                prefs_idioma: "Language (i18n)",
                prefs_tema: "GTK4 Theme (src/lib.rs)",
                prefs_tipografia: "Typography",
                prefs_ajustes: "Editor Settings",
                prefs_largura_tab: "Tab width",
                prefs_espacos_tab: "Insert spaces instead of Tab",
                prefs_auto_indentar: "Auto-indent",
                prefs_destacar_linha: "Highlight current line",
                prefs_guardar_auto: "Auto-save",
                guardar_defs: "Save settings",
            },
            Idioma::Es => Self {
                sem_nome: "Sin título",
                menu_ficheiro: "Archivo",
                menu_editar: "Editar",
                menu_ver: "Ver",
                novo: "Nuevo",
                abrir: "Abrir…",
                guardar: "Guardar",
                guardar_como: "Guardar como…",
                fechar: "Cerrar",
                sair: "Salir",
                desfazer: "Deshacer",
                refazer: "Rehacer",
                procurar: "Buscar",
                substituir: "Reemplazar",
                substituir_tudo: "Reemplazar todo",
                anterior: "Anterior",
                seguinte: "Siguiente",
                resultados: "resultados",
                procurar_substituir: "Buscar / Reemplazar",
                preferencias: "Preferencias…",
                numeros_linha: "Números de línea",
                quebra_linha: "Ajuste de línea",
                barra_estado: "Barra de estado",
                zoom_mais: "Aumentar fuente",
                zoom_menos: "Reducir fuente",
                prefs_titulo: "Preferencias de RustPad",
                prefs_idioma: "Idioma (i18n)",
                prefs_tema: "Tema GTK4 (src/lib.rs)",
                prefs_tipografia: "Tipografía",
                prefs_ajustes: "Ajustes del Editor",
                prefs_largura_tab: "Ancho de Tab",
                prefs_espacos_tab: "Insertar espacios en Tab",
                prefs_auto_indentar: "Auto-indentación",
                prefs_destacar_linha: "Resaltar línea actual",
                prefs_guardar_auto: "Guardado automático",
                guardar_defs: "Guardar ajustes",
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub modo_tema: ModoTema,
    pub idioma: Idioma,
    pub fonte_familia: String,
    pub tamanho_fonte: f32,
    pub tamanho_menu: f32,
    pub quebra_linha: bool,
    pub numeros_linha: bool,
    pub barra_estado: bool,
    pub largura_tab: u8,
    pub usar_espacos_tab: bool,
    pub auto_indentar: bool,
    pub destacar_linha_atual: bool,
    pub guardar_automatico: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modo_tema: ModoTema::Claro,
            idioma: Idioma::Pt,
            fonte_familia: String::from("Monospace"),
            tamanho_fonte: 15.0,
            tamanho_menu: 10.0,
            quebra_linha: true,
            numeros_linha: true,
            barra_estado: true,
            largura_tab: 4,
            usar_espacos_tab: true,
            auto_indentar: true,
            destacar_linha_atual: true,
            guardar_automatico: false,
        }
    }
}

impl Config {
    pub fn caminho_ficheiro() -> Option<PathBuf> {
        let base = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("rustpad").join("config.txt"))
    }

    pub fn carregar() -> Self {
        let mut cfg = Self::default();
        let Some(caminho) = Self::caminho_ficheiro() else {
            return cfg;
        };
        let Ok(conteudo) = fs::read_to_string(caminho) else {
            return cfg;
        };

        for linha in conteudo.lines() {
            let limpa = linha.trim();
            if limpa.is_empty() || limpa.starts_with('#') {
                continue;
            }
            if let Some((chave, valor)) = limpa.split_once('=') {
                let k = chave.trim();
                let v = valor.trim();
                match k {
                    "modo_tema" => {
                        cfg.modo_tema = match v {
                            "Claro" => ModoTema::Claro,
                            "Classico" => ModoTema::Classico,
                            _ => ModoTema::Escuro,
                        };
                    }
                    "idioma" => cfg.idioma = Idioma::de_codigo(v),
                    "fonte_familia" => {
                        if !v.is_empty() {
                            cfg.fonte_familia = v.to_owned();
                        }
                    }
                    "tamanho_fonte" => {
                        if let Ok(n) = v.parse::<f32>() {
                            if n.is_finite() {
                                cfg.tamanho_fonte = n.clamp(9.0, 32.0);
                            }
                        }
                    }
                    "tamanho_menu" => {
                        if let Ok(n) = v.parse::<f32>() {
                            if n.is_finite() {
                                cfg.tamanho_menu = n.clamp(10.0, 26.0);
                            }
                        }
                    }
                    "quebra_linha" => cfg.quebra_linha = v == "true",
                    "numeros_linha" => cfg.numeros_linha = v == "true",
                    "barra_estado" => cfg.barra_estado = v == "true",
                    "largura_tab" => {
                        if let Ok(n) = v.parse::<u8>() {
                            cfg.largura_tab = n.clamp(2, 8);
                        }
                    }
                    "usar_espacos_tab" => cfg.usar_espacos_tab = v == "true",
                    "auto_indentar" => cfg.auto_indentar = v == "true",
                    "destacar_linha_atual" => cfg.destacar_linha_atual = v == "true",
                    "guardar_automatico" => cfg.guardar_automatico = v == "true",
                    _ => {}
                }
            }
        }
        cfg
    }

    pub fn guardar(&self) -> Result<(), String> {
        let caminho = Self::caminho_ficheiro()
            .ok_or_else(|| String::from("Diretoria de configuração indisponível"))?;
        if let Some(pai) = caminho.parent() {
            fs::create_dir_all(pai).map_err(|e| e.to_string())?;
        }
        let modo_str = match self.modo_tema {
            ModoTema::Claro => "Claro",
            ModoTema::Escuro => "Escuro",
            ModoTema::Classico => "Classico",
        };
        let texto = format!(
            "modo_tema={modo_str}\nidioma={}\nfonte_familia={}\ntamanho_fonte={:.1}\ntamanho_menu={:.1}\nquebra_linha={}\nnumeros_linha={}\nbarra_estado={}\nlargura_tab={}\nusar_espacos_tab={}\nauto_indentar={}\ndestacar_linha_atual={}\nguardar_automatico={}\n",
            self.idioma.codigo(),
            self.fonte_familia,
            self.tamanho_fonte,
            self.tamanho_menu,
            self.quebra_linha,
            self.numeros_linha,
            self.barra_estado,
            self.largura_tab,
            self.usar_espacos_tab,
            self.auto_indentar,
            self.destacar_linha_atual,
            self.guardar_automatico
        );
        fs::write(caminho, texto).map_err(|e| e.to_string())
    }
}
