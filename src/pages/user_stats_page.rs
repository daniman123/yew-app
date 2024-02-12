use crate::components::features::user_stats::UserStats;
use yew::prelude::*;

#[function_component]
pub fn UserStatsPage() -> Html {
    html! {
        <>
            <UserStats/>
        </>
    }
}
