mod app;
mod components;
mod types;
mod hooks;
mod pages;
mod services;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new().set_max_level(tracing::Level::DEBUG).build(),
    );

    yew::Renderer::<App>::new().render();
}
