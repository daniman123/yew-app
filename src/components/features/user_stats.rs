use yew::prelude::*;

use crate::utils::hooks::use_meditation_state::use_meditation_data;

#[derive(PartialEq, Properties)]
pub struct UserStatsProps {}

#[function_component]
pub fn UserStats(props: &UserStatsProps) -> Html {
    let UserStatsProps {} = props;

    let meditation_data = use_meditation_data();

    html! {
        <div>
            <div id="introductions">
                {"Meditation Streak: "}{meditation_data.days_meditated_in_row as i64}
                {"Total Hours Meditated: "}{meditation_data.total_hours_meditated as i64}
                {"Average Duration: "}{meditation_data.average_duration_per_meditation as i64}
                {"Total Sessions: "}{meditation_data.total_meditation_sessions as i64}
                {"Favourite Category: "}{meditation_data.favorite_category.to_string()}
                {"Favourite Speaker: "}{meditation_data.favorite_speaker.to_string()}
            </div>
        </div>
    }
}
