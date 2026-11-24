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
    pub tema_claro: &'static str,
    pub tema_escuro: &'static str,
    pub tema_sistema: &'static str,
    pub prefs_icones: &'static str,
    pub prefs_pasta_icones: &'static str,
    pub prefs_detetar_xfce: &'static str,
    pub prefs_decoracoes_sistema: &'static str,
    pub prefs_tipografia: &'static str,
    pub prefs_familia: &'static str,
    pub prefs_tam_editor: &'static str,
    pub prefs_tam_menu: &'static str,
    pub prefs_ajustes: &'static str,
    pub prefs_largura_tab: &'static str,
    pub prefs_espacos_tab: &'static str,
    pub prefs_auto_indentar: &'static str,
    pub prefs_destacar_linha: &'static str,
    pub prefs_guardar_auto: &'static str,
    pub aba_aparencia: &'static str,
    pub aba_editor: &'static str,
    pub aba_sistema: &'static str,
    pub maiusculas_exatas: &'static str,
    pub dica_procurar: &'static str,
    pub dica_substituir: &'static str,
    pub dialogo_abrir_titulo: &'static str,
    pub dialogo_guardar_titulo: &'static str,
    pub dialogo_erro_titulo: &'static str,
    pub dialogo_caminho: &'static str,
    pub confirmar: &'static str,
    pub cancelar: &'static str,
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
                prefs_idioma: "Idioma",
                prefs_tema: "Tema",
                tema_claro: "Claro",
                tema_escuro: "Escuro",
                tema_sistema: "Tema do Sistema",
                prefs_icones: "Ícones e Sistema de Janelas",
                prefs_pasta_icones: "Pasta de ícones",
                prefs_detetar_xfce: "Detetar XFCE",
                prefs_decoracoes_sistema: "Usar barra de título do sistema (XFCE)",
                prefs_tipografia: "Tipografia",
                prefs_familia: "Família",
                prefs_tam_editor: "Tamanho do editor",
                prefs_tam_menu: "Tamanho dos menus",
                prefs_ajustes: "Ajustes do Editor",
                prefs_largura_tab: "Largura do Tab",
                prefs_espacos_tab: "Inserir espaços em vez de Tab",
                prefs_auto_indentar: "Auto-indentação",
                prefs_destacar_linha: "Destacar linha atual",
                prefs_guardar_auto: "Guardar automaticamente",
                aba_aparencia: "Aparência",
                aba_editor: "Editor",
                aba_sistema: "Sistema",
                maiusculas_exatas: "Distinguir maiúsculas",
                dica_procurar: "Texto a procurar…",
                dica_substituir: "Substituir por…",
                dialogo_abrir_titulo: "Abrir ficheiro",
                dialogo_guardar_titulo: "Guardar ficheiro como",
                dialogo_erro_titulo: "Erro de ficheiro",
                dialogo_caminho: "Caminho do ficheiro",
                confirmar: "Confirmar",
                cancelar: "Cancelar",
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
                prefs_idioma: "Language",
                prefs_tema: "Theme",
                tema_claro: "Light",
                tema_escuro: "Dark",
                tema_sistema: "System Theme",
                prefs_icones: "Icons & Window Manager",
                prefs_pasta_icones: "Icons folder",
                prefs_detetar_xfce: "Detect XFCE",
                prefs_decoracoes_sistema: "Use system window titlebar (XFCE)",
                prefs_tipografia: "Typography",
                prefs_familia: "Font family",
                prefs_tam_editor: "Editor font size",
                prefs_tam_menu: "Menu font size",
                prefs_ajustes: "Editor Settings",
                prefs_largura_tab: "Tab width",
                prefs_espacos_tab: "Insert spaces instead of Tab",
                prefs_auto_indentar: "Auto-indent",
                prefs_destacar_linha: "Highlight current line",
                prefs_guardar_auto: "Auto-save",
                aba_aparencia: "Appearance",
                aba_editor: "Editor",
                aba_sistema: "System",
                maiusculas_exatas: "Match case",
                dica_procurar: "Text to find…",
                dica_substituir: "Replace with…",
                dialogo_abrir_titulo: "Open file",
                dialogo_guardar_titulo: "Save file as",
                dialogo_erro_titulo: "File error",
                dialogo_caminho: "File path",
                confirmar: "Confirm",
                cancelar: "Cancel",
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
                prefs_idioma: "Idioma",
                prefs_tema: "Tema",
                tema_claro: "Claro",
                tema_escuro: "Oscuro",
                tema_sistema: "Tema del Sistema",
                prefs_icones: "Iconos y Gestor de Ventanas",
                prefs_pasta_icones: "Carpeta de iconos",
                prefs_detetar_xfce: "Detectar XFCE",
                prefs_decoracoes_sistema: "Usar barra de título del sistema (XFCE)",
                prefs_tipografia: "Tipografía",
                prefs_familia: "Familia",
                prefs_tam_editor: "Tamaño del editor",
                prefs_tam_menu: "Tamaño de menús",
                prefs_ajustes: "Ajustes del Editor",
                prefs_largura_tab: "Ancho de Tab",
                prefs_espacos_tab: "Insertar espacios en Tab",
                prefs_auto_indentar: "Auto-indentación",
                prefs_destacar_linha: "Resaltar línea actual",
                prefs_guardar_auto: "Guardado automático",
                aba_aparencia: "Apariencia",
                aba_editor: "Editor",
                aba_sistema: "Sistema",
                maiusculas_exatas: "Coincidir mayúsculas",
                dica_procurar: "Texto a buscar…",
                dica_substituir: "Reemplazar con…",
                dialogo_abrir_titulo: "Abrir archivo",
                dialogo_guardar_titulo: "Guardar archivo como",
                dialogo_erro_titulo: "Error de archivo",
                dialogo_caminho: "Ruta del archivo",
                confirmar: "Confirmar",
                cancelar: "Cancelar",
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
    pub pasta_icones: String,
    pub decoracoes_sistema: bool,
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
            modo_tema: ModoTema::Sistema,
            idioma: Idioma::Pt,
            fonte_familia: String::from("Monospace"),
            tamanho_fonte: 14.0_f32,
            tamanho_menu: 14.0_f32,
            pasta_icones: String::from("/usr/local/share/icons/GartoonRedux/24x24/actions"),
            decoracoes_sistema: true,
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
                            "Escuro" => ModoTema::Escuro,
                            _ => ModoTema::Sistema,
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
                                cfg.tamanho_fonte = n.clamp(9.0_f32, 32.0_f32);
                            }
                        }
                    }
                    "tamanho_menu" => {
                        if let Ok(n) = v.parse::<f32>() {
                            if n.is_finite() {
                                cfg.tamanho_menu = n.clamp(10.0_f32, 26.0_f32);
                            }
                        }
                    }
                    "pasta_icones" => {
                        if !v.is_empty() {
                            cfg.pasta_icones = v.to_owned();
                        }
                    }
                    "decoracoes_sistema" => cfg.decoracoes_sistema = v == "true",
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
            ModoTema::Sistema => "Sistema",
        };
        let tam_fonte = if self.tamanho_fonte.is_finite() {
            self.tamanho_fonte.clamp(9.0_f32, 32.0_f32)
        } else {
            14.0_f32
        };
        let tam_menu = if self.tamanho_menu.is_finite() {
            self.tamanho_menu.clamp(10.0_f32, 26.0_f32)
        } else {
            16.0_f32
        };
        let largura_tab = self.largura_tab.clamp(2, 8);
        let texto = format!(
            "modo_tema={modo_str}\nidioma={}\nfonte_familia={}\ntamanho_fonte={:.1}\ntamanho_menu={:.1}\npasta_icones={}\ndecoracoes_sistema={}\nquebra_linha={}\nnumeros_linha={}\nbarra_estado={}\nlargura_tab={}\nusar_espacos_tab={}\nauto_indentar={}\ndestacar_linha_atual={}\nguardar_automatico={}\n",
            self.idioma.codigo(),
            self.fonte_familia,
            tam_fonte,
            tam_menu,
            self.pasta_icones,
            self.decoracoes_sistema,
            self.quebra_linha,
            self.numeros_linha,
            self.barra_estado,
            largura_tab,
            self.usar_espacos_tab,
            self.auto_indentar,
            self.destacar_linha_atual,
            self.guardar_automatico
        );
        fs::write(caminho, texto).map_err(|e| e.to_string())
    }
}
