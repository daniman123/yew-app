use yew::prelude::*;

use crate::pages::user_stats_page::UserStats;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserStats/>
    }
}
