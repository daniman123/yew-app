mod app;
pub mod components;
mod pages;
pub mod services;
pub mod utils;
use app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();
}
