use yew::prelude::*;

use crate::pages::user_stats_page::UserStatsPage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-[844px] w-[390px] border">
            <UserStatsPage/>
        </div>
    }
}
