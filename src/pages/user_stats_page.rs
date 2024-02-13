use crate::components::{
    features::user_stats::UserStats, layouts::primary_layout::PrimaryLayout,
    ui::title_banner::TitleBanner,
};
use yew::prelude::*;

#[function_component]
pub fn UserStatsPage() -> Html {
    html! {
        <>
            <PrimaryLayout>
                <TitleBanner title={"Stats"} />
                <UserStats/>
            </PrimaryLayout>
        </>
    }
}
