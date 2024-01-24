use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct UserStatsProps {}

#[function_component]
pub fn UserStats(props: &UserStatsProps) -> Html {
    let UserStatsProps {} = props;
    html! {
        <h1>{"UserStats"}</h1>
    }
}
