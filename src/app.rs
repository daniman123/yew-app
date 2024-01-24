use yew::prelude::*;

use crate::pages::landing_page::LandingPage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <LandingPage/>
    }
}
