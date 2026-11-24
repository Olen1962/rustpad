#![forbid(unsafe_code)]

mod app;
mod barra;
mod config;
mod editar;
mod estilo;
mod ficheiro;
mod fontes;
mod preferencias;
mod procura;
mod ver;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let config = config::Config::carregar();

    let opcoes = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0_f32, 650.0_f32])
            .with_min_inner_size([350.0_f32, 250.0_f32])
            // Por omissão usa o gestor de janelas do sistema (XFCE / xfwm4),
            // mantendo a barra de menus separada da barra de título.
            .with_decorations(config.decoracoes_sistema)
            .with_title("RustPad"),
        ..Default::default()
    };

    eframe::run_native(
        "RustPad",
        opcoes,
        Box::new(move |criacao| {
            egui_extras::install_image_loaders(&criacao.egui_ctx);
            estilo::aplicar_fontes(&criacao.egui_ctx, &config);
            estilo::aplicar_tema(&criacao.egui_ctx, &config);
            Ok(Box::new(app::Editor::novo(config)))
        }),
    )
}
