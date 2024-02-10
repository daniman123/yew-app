use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::utils::database::meditation_log::{
    meditation_data_builder::MeditationData, read_write_meditation_data::read_meditation_data,
};

#[derive(PartialEq, Properties)]
pub struct UserStatsProps {}

#[function_component]
pub fn UserStats(props: &UserStatsProps) -> Html {
    let UserStatsProps {} = props;

    let data_state: UseStateHandle<Vec<MeditationData>> = use_state(|| {
        vec![MeditationData {
            ..Default::default()
        }]
    });

    let get_data = {
        let data_state = data_state.clone();
        Callback::from(move |_| {
            let data = read_meditation_data();
            data_state.set(data);

            // web_sys::console::log_1(&format!("MyStruct: {:?}", data).into());
            web_sys::console::log_1(&JsValue::from_str("data clicekd"));
        })
    };

    html! {
        <div>
            <button onclick={get_data} >{"Get Data!"}</button>

            <div id="introductions">
        {
            (*data_state).clone().into_iter().map(|data| {
                let MeditationData {category,datetime,duration,speaker} = data;
                html!{
                    <div key={&*category} style={"display: flex; gap: 10px;"}>
                        <div>{"speaker: "} { speaker }</div>
                        <div>{"category: "} { category }</div>
                        <div>{"datetime: "} { datetime }</div>
                        <div>{"duration: "} { duration }</div>
                    </div>
                }
            }).collect::<Html>()
        }
    </div>
        </div>
    }
}
