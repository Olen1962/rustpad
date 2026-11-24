#![allow(dead_code)]

/// Tipo de cor RGB `(R, G, B)` com valores de 0 a 255.
pub type Rgb = (u8, u8, u8);

/// Extensão utilitária para converter tuplos `(u8, u8, u8)` diretamente
/// para formatos compatíveis com `egui`, `gtk4` / `cairo` e shaders, sem dependências externas.
pub trait CorExt {
    /// Converte para `(u8, u8, u8)` (ex: `egui::Color32::from_rgb(r, g, b)`).
    fn para_rgb_u8(self) -> (u8, u8, u8);

    /// Converte para `(u8, u8, u8, u8)` com canal alfa (ex: `egui::Color32::from_rgba_unmultiplied`).
    fn para_rgba_u8(self, alfa: u8) -> (u8, u8, u8, u8);

    /// Converte para `(f64, f64, f64)` normalizado entre `0.0` e `1.0` (ideal para Cairo / GTK4).
    fn para_f64(self) -> (f64, f64, f64);

    /// Converte para `(f64, f64, f64, f64)` com alfa (ideal para `cairo::Context::set_source_rgba`).
    fn para_rgba_f64(self, alfa: f64) -> (f64, f64, f64, f64);

    /// Converte para `(f32, f32, f32)` normalizado entre `0.0` e `1.0` (ideal para shaders / wgpu).
    fn para_f32(self) -> (f32, f32, f32);

    /// Converte para `(f32, f32, f32, f32)` normalizado com alfa.
    fn para_rgba_f32(self, alfa: f32) -> (f32, f32, f32, f32);

    /// Converte para um inteiro hexadecimal `0xRRGGBB`.
    fn para_hex_u32(self) -> u32;

    /// Mistura esta cor com outra (`fator` entre `0.0` e `1.0`).
    fn misturar(self, outra: Rgb, fator: f32) -> Rgb;
}

impl CorExt for Rgb {
    #[inline]
    fn para_rgb_u8(self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    #[inline]
    fn para_rgba_u8(self, alfa: u8) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, alfa)
    }

    #[inline]
    fn para_f64(self) -> (f64, f64, f64) {
        (
            self.0 as f64 / 255.0,
            self.1 as f64 / 255.0,
            self.2 as f64 / 255.0,
        )
    }

    #[inline]
    fn para_rgba_f64(self, alfa: f64) -> (f64, f64, f64, f64) {
        (
            self.0 as f64 / 255.0,
            self.1 as f64 / 255.0,
            self.2 as f64 / 255.0,
            alfa.clamp(0.0, 1.0),
        )
    }

    #[inline]
    fn para_f32(self) -> (f32, f32, f32) {
        (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        )
    }

    #[inline]
    fn para_rgba_f32(self, alfa: f32) -> (f32, f32, f32, f32) {
        (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
            alfa.clamp(0.0, 1.0),
        )
    }

    #[inline]
    fn para_hex_u32(self) -> u32 {
        ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }

    #[inline]
    fn misturar(self, outra: Rgb, fator: f32) -> Rgb {
        let t = fator.clamp(0.0, 1.0);
        let inv = 1.0 - t;
        (
            (self.0 as f32 * inv + outra.0 as f32 * t).round() as u8,
            (self.1 as f32 * inv + outra.1 as f32 * t).round() as u8,
            (self.2 as f32 * inv + outra.2 as f32 * t).round() as u8,
        )
    }
}

/// Escolhe texto escuro ou branco conforme o brilho do fundo, para ficar sempre legível.
pub fn texto_sobre(fundo: Rgb) -> Rgb {
    let brilho = 0.299 * fundo.0 as f32 + 0.587 * fundo.1 as f32 + 0.114 * fundo.2 as f32;
    if brilho > 160.0 {
        paleta::TEXTO_ESCURO
    } else {
        paleta::BRANCO_PURO
    }
}

/// Converte uma cor para o formato CSS `#rrggbb`.
pub fn para_hex_css(cor: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", cor.0, cor.1, cor.2)
}

// ============================================================================
// SISTEMA DE TEMAS SEMÂNTICOS (Compatível com GTK4 / XFCE & egui)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModoTema {
    Claro,
    Escuro,
    Sistema,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tema {
    pub modo: ModoTema,
    pub fundo_janela: Rgb,
    pub fundo_cartao: Rgb,
    pub fundo_texto: Rgb,
    pub barra_titulo: Rgb,
    pub popover: Rgb,
    pub borda: Rgb,
    pub separador: Rgb,
    pub texto_principal: Rgb,
    pub texto_secundario: Rgb,
    pub texto_desativado: Rgb,
    pub destaque: Rgb,
    pub destaque_hover: Rgb,
    pub destaque_clique: Rgb,
    pub sucesso: Rgb,
    pub aviso: Rgb,
    pub perigo: Rgb,
    pub raio_widget: u8,
    pub raio_janela: u8,
}

impl Tema {
    pub const CLARO: Self = Self {
        modo: ModoTema::Claro,
        fundo_janela: paleta::JANELA_FUNDO_CLARO,
        fundo_cartao: paleta::CARTAO_CLARO,
        fundo_texto: paleta::BRANCO_PURO,
        barra_titulo: paleta::BARRA_TITULO_CLARA,
        popover: paleta::BRANCO_PURO,
        borda: paleta::BORDA_CLARA,
        separador: paleta::SEPARADOR_CLARO,
        texto_principal: paleta::TEXTO_ESCURO,
        texto_secundario: paleta::TEXTO_SECUNDARIO_CLARO,
        texto_desativado: paleta::TEXTO_DESATIVADO_CLARO,
        destaque: paleta::AZUL_GTK,
        destaque_hover: paleta::AZUL_HOVER,
        destaque_clique: paleta::AZUL_CLIQUE,
        sucesso: paleta::VERDE_GTK,
        aviso: paleta::LARANJA_GTK,
        perigo: paleta::VERMELHO_GTK,
        raio_widget: 6,
        raio_janela: 12,
    };

    pub const ESCURO: Self = Self {
        modo: ModoTema::Escuro,
        fundo_janela: paleta::JANELA_FUNDO_ESCURO,
        fundo_cartao: paleta::CARTAO_ESCURO,
        fundo_texto: paleta::FUNDO_ESCURO,
        barra_titulo: paleta::BARRA_TITULO_ESCURA,
        popover: paleta::POPOVER_ESCURO,
        borda: paleta::BORDA_ESCURA,
        separador: paleta::SEPARADOR_ESCURO,
        texto_principal: paleta::BRANCO_PURO,
        texto_secundario: paleta::TEXTO_SECUNDARIO_ESCURO,
        texto_desativado: paleta::TEXTO_DESATIVADO_ESCURO,
        destaque: paleta::AZUL_GTK,
        destaque_hover: paleta::AZUL_HOVER,
        destaque_clique: paleta::AZUL_CLIQUE,
        sucesso: paleta::VERDE_GTK,
        aviso: paleta::LARANJA_GTK,
        perigo: paleta::VERMELHO_GTK,
        raio_widget: 6,
        raio_janela: 12,
    };

    pub const SISTEMA: Self = Self {
        modo: ModoTema::Sistema,
        fundo_janela: (245, 245, 244),
        fundo_cartao: paleta::CARTAO_CLARO,
        fundo_texto: paleta::BRANCO_PURO,
        barra_titulo: (44, 44, 44),
        popover: paleta::BRANCO_PURO,
        borda: paleta::BORDA_CLARA,
        separador: paleta::SEPARADOR_CLARO,
        texto_principal: paleta::TEXTO_ESCURO,
        texto_secundario: paleta::TEXTO_SECUNDARIO_CLARO,
        texto_desativado: paleta::TEXTO_DESATIVADO_CLARO,
        destaque: paleta::AZUL_GTK,
        destaque_hover: paleta::AZUL_HOVER,
        destaque_clique: paleta::AZUL_CLIQUE,
        sucesso: paleta::TANGO_CAMALEAO_ESCURO,
        aviso: paleta::TANGO_LARANJA,
        perigo: paleta::TANGO_ESCARLATE,
        raio_widget: 4,
        raio_janela: 6,
    };

    #[inline]
    pub const fn obter(modo: ModoTema) -> Self {
        match modo {
            ModoTema::Claro => Self::CLARO,
            ModoTema::Escuro => Self::ESCURO,
            ModoTema::Sistema => Self::SISTEMA,
        }
    }

    /// Calcula as 3 variações de fundo de botão (inativo, hover, ativo) estilo GTK4.
    #[inline]
    pub fn estados_botao(&self) -> (Rgb, Rgb, Rgb) {
        let inativo = self.fundo_cartao.misturar(self.borda, 0.25);
        let hover = self.fundo_cartao.misturar(self.borda, 0.45);
        let ativo = self.fundo_cartao.misturar(self.borda, 0.65);
        (inativo, hover, ativo)
    }

    /// Gera o CSS do GTK4 com as cores deste tema.
    /// É só texto, por isso a biblioteca continua sem dependências.
    pub fn para_css(&self) -> String {
        let (btn, btn_hover, btn_ativo) = self.estados_botao();
        let perigo_hover = self.perigo.misturar(paleta::BRANCO_PURO, 0.12);
        let perigo_clique = self.perigo.misturar(paleta::ESCURO_5, 0.25);
        let h = para_hex_css;

        let pares: Vec<(&str, String)> = vec![
            ("%JANELA%", h(self.fundo_janela)),
            ("%CARTAO%", h(self.fundo_cartao)),
            ("%TEXTO_FUNDO%", h(self.fundo_texto)),
            ("%BARRA%", h(self.barra_titulo)),
            ("%POPOVER%", h(self.popover)),
            ("%BORDA%", h(self.borda)),
            ("%SEPARADOR%", h(self.separador)),
            ("%TEXTO%", h(self.texto_principal)),
            ("%TEXTO2%", h(self.texto_secundario)),
            ("%DESTAQUE%", h(self.destaque)),
            ("%DESTAQUE_HOVER%", h(self.destaque_hover)),
            ("%DESTAQUE_CLIQUE%", h(self.destaque_clique)),
            ("%TEXTO_DESTAQUE%", h(texto_sobre(self.destaque))),
            ("%PERIGO%", h(self.perigo)),
            ("%PERIGO_HOVER%", h(perigo_hover)),
            ("%PERIGO_CLIQUE%", h(perigo_clique)),
            ("%TEXTO_PERIGO%", h(texto_sobre(self.perigo))),
            ("%BTN%", h(btn)),
            ("%BTN_HOVER%", h(btn_hover)),
            ("%BTN_ATIVO%", h(btn_ativo)),
            ("%RAIO%", self.raio_widget.to_string()),
            ("%RAIO_BOTAO%", (self.raio_widget + 2).to_string()),
        ];

        let mut css = MODELO_CSS.to_string();
        for (chave, valor) in pares {
            css = css.replace(chave, &valor);
        }
        css
    }
}

/// Modelo do CSS: as palavras entre % são trocadas pelas cores do tema.
const MODELO_CSS: &str = r#"
@define-color window_bg_color %JANELA%;
@define-color window_fg_color %TEXTO%;
@define-color view_bg_color %TEXTO_FUNDO%;
@define-color view_fg_color %TEXTO%;
@define-color headerbar_bg_color %BARRA%;
@define-color headerbar_fg_color %TEXTO%;
@define-color popover_bg_color %POPOVER%;
@define-color popover_fg_color %TEXTO%;
@define-color dialog_bg_color %JANELA%;
@define-color dialog_fg_color %TEXTO%;
@define-color accent_bg_color %DESTAQUE%;
@define-color accent_fg_color %TEXTO_DESTAQUE%;
@define-color accent_color %DESTAQUE%;
@define-color destructive_bg_color %PERIGO%;
@define-color destructive_fg_color %TEXTO_PERIGO%;
@define-color borders %BORDA%;
@define-color theme_bg_color %JANELA%;
@define-color theme_fg_color %TEXTO%;
@define-color theme_base_color %TEXTO_FUNDO%;
@define-color theme_text_color %TEXTO%;
@define-color theme_selected_bg_color %DESTAQUE%;
@define-color theme_selected_fg_color %TEXTO_DESTAQUE%;

window, window.background {
    background-color: %JANELA%;
    color: %TEXTO%;
}

menubar {
    background-color: %BARRA%;
    color: %TEXTO%;
    border-bottom: 1px solid %SEPARADOR%;
}
menubar > item:hover {
    background-color: %BTN_HOVER%;
}

popover.menu contents, popover contents {
    background-color: %POPOVER%;
    color: %TEXTO%;
    border: 1px solid %BORDA%;
    border-radius: %RAIO%px;
}
modelbutton:hover {
    background-color: %BTN_HOVER%;
}

textview, textview text {
    background-color: %TEXTO_FUNDO%;
    color: %TEXTO%;
}
textview text selection, textview selection {
    background-color: %DESTAQUE%;
    color: %TEXTO_DESTAQUE%;
}

entry {
    background-color: %TEXTO_FUNDO%;
    color: %TEXTO%;
    border: 1px solid %BORDA%;
    border-radius: %RAIO%px;
    box-shadow: none;
}
entry:focus-within {
    border-color: %DESTAQUE%;
}

check {
    background-image: none;
    background-color: %TEXTO_FUNDO%;
    border: 1px solid %BORDA%;
}
check:checked {
    background-color: %DESTAQUE%;
    color: %TEXTO_DESTAQUE%;
    border-color: %DESTAQUE%;
}

separator {
    background-color: %SEPARADOR%;
}
.dim-label {
    color: %TEXTO2%;
}
scrollbar slider {
    background-color: %BORDA%;
}
scrollbar slider:hover {
    background-color: %TEXTO2%;
}

/* Botões */
button {
    background-image: none;
    background-color: %BTN%;
    color: %TEXTO%;
    border: 1px solid %BORDA%;
    border-radius: %RAIO_BOTAO%px;
    box-shadow: none;
    text-shadow: none;
}
button.text-button {
    padding: 8px 20px;
}
button:hover {
    background-color: %BTN_HOVER%;
}
button:active, button:checked {
    background-color: %BTN_ATIVO%;
}
button:focus-visible {
    outline: 2px solid %DESTAQUE_HOVER%;
    outline-offset: 2px;
}

/* Botão de destaque (azul), com borda de contraste */
button.suggested-action {
    background-color: %DESTAQUE%;
    color: %TEXTO_DESTAQUE%;
    border-color: %TEXTO%;
}
button.suggested-action:hover {
    background-color: %DESTAQUE_HOVER%;
}
button.suggested-action:active {
    background-color: %DESTAQUE_CLIQUE%;
}

/* Botão de ação destrutiva (vermelho) */
button.destructive-action {
    background-color: %PERIGO%;
    color: %TEXTO_PERIGO%;
    border-color: %TEXTO%;
}
button.destructive-action:hover {
    background-color: %PERIGO_HOVER%;
}
button.destructive-action:active {
    background-color: %PERIGO_CLIQUE%;
}
"#;

// ============================================================================
// PALETA RGB REUTILIZÁVEL (82 cores base estilo GTK / Adwaita e temas clássicos)
// ============================================================================
pub mod paleta {
    use super::Rgb;

    // --- Azuis base ---
    pub const AZUL_GTK: Rgb = (53, 132, 228); // #3584e4
    pub const AZUL_HOVER: Rgb = (67, 141, 230); // #438de6
    pub const AZUL_CLIQUE: Rgb = (28, 109, 208); // #1c6dd0
    pub const AZUL_ESCURO: Rgb = (24, 78, 145); // #184e91

    // --- Verdes e avisos base ---
    pub const VERDE_GTK: Rgb = (46, 194, 126); // #2ec27e
    pub const VERDE_ESCURO: Rgb = (30, 126, 82); // #1e7e52
    pub const AMARELO_GTK: Rgb = (246, 211, 45); // #f6d32d
    pub const LARANJA_GTK: Rgb = (255, 120, 0); // #ff7800

    // --- Vermelhos e neutros base ---
    pub const VERMELHO_GTK: Rgb = (224, 27, 36); // #e01b24
    pub const VERMELHO_ESCURO: Rgb = (150, 18, 24); // #961218
    pub const BRANCO_PURO: Rgb = (255, 255, 255); // #ffffff
    pub const TEXTO_CLARO: Rgb = (225, 225, 225); // #e1e1e1

    // --- Cinzentos e fundos base ---
    pub const CINZENTO_CLARO: Rgb = (154, 153, 150); // #9a9996
    pub const CINZENTO_MEDIO: Rgb = (94, 92, 100); // #5e5c64
    pub const FUNDO_ESCURO: Rgb = (30, 30, 30); // #1e1e1e
    pub const PRETO_SUAVE: Rgb = (20, 20, 22); // #141416
    pub const FUNDO_CLARO: Rgb = (246, 245, 244); // #f6f5f4
    pub const TEXTO_ESCURO: Rgb = (36, 36, 36); // #242424

    // --- Paleta Adwaita / GNOME HIG (Gradações 1 a 5) ---
    pub const AZUL_1: Rgb = (153, 193, 241); // #99c1f1
    pub const AZUL_2: Rgb = (98, 160, 234); // #62a0ea
    pub const AZUL_5: Rgb = (26, 95, 180); // #1a5fb4

    pub const VERDE_1: Rgb = (143, 240, 164); // #8ff0a4
    pub const VERDE_2: Rgb = (87, 227, 137); // #57e389
    pub const VERDE_4: Rgb = (38, 162, 105); // #26a269

    pub const AMARELO_1: Rgb = (249, 240, 107); // #f9f06b
    pub const AMARELO_2: Rgb = (248, 228, 92); // #f8e45c
    pub const AMARELO_4: Rgb = (229, 165, 10); // #e5a50a
    pub const AMARELO_5: Rgb = (181, 131, 90); // #b5835a

    pub const LARANJA_1: Rgb = (255, 190, 111); // #ffbe6f
    pub const LARANJA_2: Rgb = (255, 163, 72); // #ffa348
    pub const LARANJA_4: Rgb = (230, 97, 0); // #e66100
    pub const LARANJA_5: Rgb = (198, 70, 0); // #c64600

    pub const VERMELHO_1: Rgb = (246, 97, 81); // #f66151
    pub const VERMELHO_2: Rgb = (237, 51, 59); // #ed333b
    pub const VERMELHO_4: Rgb = (192, 28, 40); // #c01c28
    pub const VERMELHO_5: Rgb = (165, 29, 36); // #a51d24
    pub const ROXO_1: Rgb = (220, 138, 221); // #dc8add
    pub const ROXO_2: Rgb = (192, 97, 203); // #c061cb
    pub const ROXO_3: Rgb = (145, 65, 172); // #9141ac
    pub const ROXO_4: Rgb = (129, 61, 156); // #813d9c
    pub const ROXO_5: Rgb = (97, 53, 131); // #613583
    pub const CASTANHO_1: Rgb = (205, 171, 143); // #cdab8f
    pub const CASTANHO_2: Rgb = (181, 131, 90); // #b5835a
    pub const CASTANHO_3: Rgb = (152, 106, 68); // #986a44
    pub const CASTANHO_4: Rgb = (134, 94, 60); // #865e3c
    pub const CASTANHO_5: Rgb = (99, 69, 44); // #63452c
    pub const CLARO_3: Rgb = (222, 221, 218); // #deddda
    pub const CLARO_4: Rgb = (192, 191, 188); // #c0bfbc
    pub const ESCURO_1: Rgb = (119, 118, 123); // #77767b
    pub const ESCURO_3: Rgb = (61, 56, 70); // #3d3846
    pub const ESCURO_4: Rgb = (36, 31, 49); // #241f31
    pub const ESCURO_5: Rgb = (0, 0, 0); // #000000
                                         // --- Superfícies, bordas e widgets GTK ---
    pub const JANELA_FUNDO_CLARO: Rgb = (250, 250, 250); // #fafafa
    pub const JANELA_FUNDO_ESCURO: Rgb = (36, 36, 36); // #242424
    pub const BARRA_TITULO_CLARA: Rgb = (235, 235, 235); // #ebebeb
    pub const BARRA_TITULO_ESCURA: Rgb = (48, 48, 48); // #303030
    pub const CARTAO_CLARO: Rgb = (255, 255, 255); // #ffffff
    pub const CARTAO_ESCURO: Rgb = (48, 48, 48); // #303030
    pub const POPOVER_ESCURO: Rgb = (56, 56, 56); // #383838
    pub const BORDA_CLARA: Rgb = (205, 199, 194); // #cdc7c2
    pub const BORDA_ESCURA: Rgb = (69, 69, 69); // #454545
    pub const SEPARADOR_CLARO: Rgb = (225, 222, 219); // #e1dedb
    pub const SEPARADOR_ESCURO: Rgb = (60, 60, 60); // #3c3c3c
    pub const TEXTO_SECUNDARIO_CLARO: Rgb = (94, 92, 100); // #5e5c64
    pub const TEXTO_SECUNDARIO_ESCURO: Rgb = (154, 153, 150); // #9a9996
    pub const TEXTO_DESATIVADO_CLARO: Rgb = (146, 149, 149); // #929595
    pub const TEXTO_DESATIVADO_ESCURO: Rgb = (104, 104, 104); // #686868
                                                             // --- Temas GTK clássicos (Tango, Clearlooks, Raleigh, Ubuntu) ---
    pub const TANGO_CEU_CLARO: Rgb = (114, 159, 207); // #729fcf
    pub const TANGO_CEU_MEDIO: Rgb = (52, 101, 164); // #3465a4
    pub const TANGO_CEU_ESCURO: Rgb = (32, 74, 135); // #204a87
    pub const TANGO_CAMALEAO_CLARO: Rgb = (138, 226, 52); // #8ae234
    pub const TANGO_CAMALEAO_MEDIO: Rgb = (115, 210, 22); // #73d216
    pub const TANGO_CAMALEAO_ESCURO: Rgb = (78, 154, 6); // #4e9a06
    pub const TANGO_MANTEIGA: Rgb = (252, 233, 79); // #fce94f
    pub const TANGO_LARANJA: Rgb = (252, 175, 62); // #fcaf3e
    pub const TANGO_ESCARLATE: Rgb = (239, 41, 41); // #ef2929
    pub const TANGO_AMEIXA: Rgb = (173, 127, 168); // #ad7fa8
    pub const CLEARLOOKS_CINZA: Rgb = (237, 236, 235); // #edeceb
    pub const CLEARLOOKS_AZUL_SEL: Rgb = (134, 171, 217); // #86abd9
    pub const RALEIGH_CINZA: Rgb = (220, 218, 213); // #dcdad5
    pub const UBUNTU_LARANJA: Rgb = (233, 84, 32); // #e95420
    pub const UBUNTU_BERINGELA: Rgb = (119, 33, 111); // #77216f
}
