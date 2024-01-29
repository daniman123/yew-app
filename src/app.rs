use yew::prelude::*;

use crate::pages::user_stats_page::UserStatsPage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserStatsPage/>
    }
}
