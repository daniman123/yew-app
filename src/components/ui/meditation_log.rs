use yew::prelude::*;

use crate::{
    components::ui::card::Card,
    utils::database::meditation_log::{
        meditation_data_builder::MeditationData, read_write_meditation_data::read_meditation_data,
    },
};

#[derive(PartialEq, Properties)]
pub struct MeditationLogProps {}

#[function_component]
pub fn MeditationLog(props: &MeditationLogProps) -> Html {
    let MeditationLogProps {} = props;

    let data_state: UseStateHandle<Vec<MeditationData>> = use_state(|| vec![]);
    let data_state_cl = data_state.clone();

    use_effect_with((), move |_| {
        let data = read_meditation_data();
        data_state_cl.set(data);
    });

    html! {
        <Card>
            <div class="w-full">
                {
                    (*data_state).clone().into_iter().rev().take(4).map(|feedback| {
                        let MeditationData {category,datetime,duration,speaker} = feedback;

                        html! {
                            <div class="grid gap-1 grid-flow-col auto-cols-fr">
                                <div class="truncate">{"category: "} {category}</div>
                                <div class="">{"datetime: "} {datetime}</div>
                                <div class="">{"duration: "} {duration}</div>
                                <div class="">{"speaker: "} {speaker}</div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </Card>
    }
}
