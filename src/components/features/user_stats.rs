use yew::prelude::*;

use crate::{components::ui::card::Card, utils::hooks::use_meditation_state::use_meditation_data};

#[derive(PartialEq, Properties)]
pub struct UserStatsProps {}

#[function_component]
pub fn UserStats(props: &UserStatsProps) -> Html {
    let UserStatsProps {} = props;

    let meditation_data = use_meditation_data();

    html! {
        <Card>
            <div class="pb-1 text-2xl font-semibold border-b">{"Meditation Streak: "}{meditation_data.days_meditated_in_row as i64}</div>
            <div id="sub-grid" class="px-4 py-1 text-sm font-semibold">
            <div class="">{"Total Hours Meditated: "}{meditation_data.total_hours_meditated as i64}</div>
                <div class="">{"Average Duration: "}{meditation_data.average_duration_per_meditation as i64}</div>
                <div class="">{"Total Sessions: "}{meditation_data.total_meditation_sessions as i64}</div>
                <div class="">{"Favourite Category: "}{meditation_data.favorite_category.to_string()}</div>
                <div class="">{"Favourite Speaker: "}{meditation_data.favorite_speaker.to_string()}</div>
            </div>
        </Card>

    }
}
