#![forbid(unsafe_code)]

use eframe::egui;
use rustpad::{CorExt, ModoTema, Rgb, Tema, texto_sobre};

use crate::config::Config;

#[inline]
pub fn cor32(rgb: Rgb) -> egui::Color32 {
    let (r, g, b) = rgb.para_rgb_u8();
    egui::Color32::from_rgb(r, g, b)
}

#[inline]
pub fn cor32_alfa(rgb: Rgb, alfa: u8) -> egui::Color32 {
    let (r, g, b, a) = rgb.para_rgba_u8(alfa);
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

pub fn aplicar_fontes(ctx: &egui::Context, config: &Config) {
    let mut estilo = (*ctx.style()).clone();
    let corpo = config.tamanho_fonte.clamp(9.0, 32.0);
    let menu = config.tamanho_menu.clamp(10.0, 26.0);

    estilo.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new((menu * 0.85).max(9.0), egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(menu, egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(menu, egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(menu * 1.2, egui::FontFamily::Proportional),
    );
    estilo.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(corpo, egui::FontFamily::Monospace),
    );

    ctx.set_style(estilo);
}

pub fn aplicar_tema(ctx: &egui::Context, config: &Config) {
    let tema = Tema::obter(config.modo_tema);
    let (btn_inativo, btn_hover, btn_ativo) = tema.estados_botao();
    let raio = egui::Rounding::same(tema.raio_widget as f32);

    let mut visuals = match tema.modo {
        ModoTema::Escuro => egui::Visuals::dark(),
        ModoTema::Claro | ModoTema::Classico => egui::Visuals::light(),
    };

    visuals.override_text_color = Some(cor32(tema.texto_principal));
    visuals.window_fill = cor32(tema.fundo_janela);
    visuals.panel_fill = cor32(tema.fundo_janela);
    visuals.extreme_bg_color = cor32(tema.fundo_texto);
    visuals.faint_bg_color = cor32(tema.barra_titulo);
    visuals.code_bg_color = cor32(tema.fundo_texto);
    visuals.hyperlink_color = cor32(tema.destaque);
    visuals.warn_fg_color = cor32(tema.aviso);
    visuals.error_fg_color = cor32(tema.perigo);
    visuals.window_rounding = egui::Rounding::same(tema.raio_janela as f32);
    visuals.window_stroke = egui::Stroke::new(1.0, cor32(tema.borda));
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::Vec2::new(0.0, 6.0),
        blur: 20.0,
        spread: 0.0,
        color: cor32_alfa((0, 0, 0), 96),
    };
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: egui::Vec2::new(0.0, 3.0),
        blur: 10.0,
        spread: 0.0,
        color: cor32_alfa((0, 0, 0), 64),
    };

    visuals.selection.bg_fill = cor32(tema.destaque);
    visuals.selection.stroke = egui::Stroke::new(1.0, cor32(texto_sobre(tema.destaque)));

    visuals.widgets.noninteractive.bg_fill = cor32(tema.fundo_janela);
    visuals.widgets.noninteractive.weak_bg_fill = cor32(tema.barra_titulo);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, cor32(tema.separador));
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, cor32(tema.texto_principal));
    visuals.widgets.noninteractive.rounding = raio;

    visuals.widgets.inactive.bg_fill = cor32(btn_inativo);
    visuals.widgets.inactive.weak_bg_fill = cor32(tema.fundo_cartao);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, cor32(tema.borda));
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, cor32(tema.texto_principal));
    visuals.widgets.inactive.rounding = raio;

    visuals.widgets.hovered.bg_fill = cor32(btn_hover);
    visuals.widgets.hovered.weak_bg_fill = cor32(btn_hover);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, cor32(tema.destaque_hover));
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, cor32(tema.texto_principal));
    visuals.widgets.hovered.rounding = raio;

    visuals.widgets.active.bg_fill = cor32(btn_ativo);
    visuals.widgets.active.weak_bg_fill = cor32(tema.destaque_clique);
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, cor32(tema.destaque));
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, cor32(texto_sobre(tema.destaque)));
    visuals.widgets.active.rounding = raio;

    visuals.widgets.open.bg_fill = cor32(tema.popover);
    visuals.widgets.open.weak_bg_fill = cor32(tema.popover);
    visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0, cor32(tema.borda));
    visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0, cor32(tema.texto_principal));
    visuals.widgets.open.rounding = raio;

    let mut estilo = (*ctx.style()).clone();
    estilo.visuals = visuals;
    estilo.spacing.item_spacing = egui::vec2(8.0, 6.0);
    estilo.spacing.button_padding = egui::vec2(12.0, 5.0);
    estilo.spacing.window_margin = egui::Margin::same(10.0);
    ctx.set_style(estilo);
}
