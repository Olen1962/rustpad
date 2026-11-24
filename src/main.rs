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
    let opcoes = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([350.0, 250.0])
            .with_decorations(false)
            .with_title("RustPad"),
        ..Default::default()
    };

    eframe::run_native(
        "RustPad",
        opcoes,
        Box::new(|criacao| {
            let config = config::Config::carregar();
            estilo::aplicar_fontes(&criacao.egui_ctx, &config);
            estilo::aplicar_tema(&criacao.egui_ctx, &config);
            Ok(Box::new(app::Editor::novo(config)))
        }),
    )
}
