use crate::components::{
    layouts::primary_layout::PrimaryLayout, ui::meditation_log::MeditationLog,
};
use yew::prelude::*;

#[function_component]
pub fn TestPage() -> Html {
    html! {
        <>
            <PrimaryLayout>
                <MeditationLog />
            </PrimaryLayout>
        </>
    }
}
