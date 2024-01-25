mod app;
mod pages;
pub mod services;
mod utils;
use app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();
}
