mod app;
mod pages;
pub mod services;
use app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();
}
