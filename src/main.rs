mod app;
mod components;
mod router;
mod pages;

fn main() {
    yew::Renderer::<app::App>::new().render();
}