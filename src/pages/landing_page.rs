use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LandingPageProps {}

#[function_component]
pub fn LandingPage(props: &LandingPageProps) -> Html {
    let LandingPageProps {} = props;
    html! {
        <h1>{"LandingPage"}</h1>
    }
}
