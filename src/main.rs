mod app;
mod components;
mod pages;
mod router;

fn main() {
    next_rs::Renderer::<app::App>::new().render();
}
