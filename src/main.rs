mod app;
mod pages;
pub mod services;
pub mod utils;
pub mod components;
use app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();
}
