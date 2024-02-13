use yew::prelude::*;

use crate::components::layouts::primary_layout::PrimaryLayout;

#[derive(PartialEq, Properties)]
pub struct LandingPageProps {}

#[function_component]
pub fn LandingPage(props: &LandingPageProps) -> Html {
    let LandingPageProps {} = props;
    html! {
        <PrimaryLayout>
            <h1>{"LandingPage"}</h1>
        </PrimaryLayout>

    }
}
