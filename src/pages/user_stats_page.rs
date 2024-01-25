// TODO - implement wrapper for client access to localStorage/database

use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_app::services::local_storage::LocalStorageService;

#[derive(PartialEq, Properties)]
pub struct UserStatsProps {}

#[function_component]
pub fn UserStats(props: &UserStatsProps) -> Html {
    let UserStatsProps {} = props;

    let input_key = use_state(|| String::new());
    let input_value = use_state(|| String::new());
    let stored_value = use_state(|| String::new());

    let on_key_change = {
        let input_key = input_key.clone();
        Callback::from(move |e: yew::events::InputEvent| {
            input_key.set(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };

    let on_value_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: yew::events::InputEvent| {
            input_value.set(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };

    let on_save = {
        let input_key = input_key.clone();
        let input_value = input_value.clone();
        Callback::from(move |_| {
            let key = input_key.as_str();
            let value = input_value.as_str();
            LocalStorageService::create(key, value).unwrap_or_else(|err| {
                web_sys::console::log_1(&JsValue::from_str(&format!("Error saving: {:?}", err)));
            });
        })
    };

    let on_load = {
        let input_key = input_key.clone();
        let stored_value = stored_value.clone();
        Callback::from(
            move |_| match LocalStorageService::read(input_key.as_str()) {
                Ok(Some(value)) => stored_value.set(value),
                Ok(None) => stored_value.set("Not found".to_string()),
                Err(err) => {
                    web_sys::console::log_1(&JsValue::from_str(&format!(
                        "Error loading: {:?}",
                        err
                    )));
                }
            },
        )
    };

    html! {
        <>
            <div>
                <input placeholder="Key" oninput={on_key_change} value={(*input_key).clone()} />
                <input placeholder="Value" oninput={on_value_change} value={(*input_value).clone()} />
                <button onclick={on_save}>{"Save"}</button>
                <button onclick={on_load}>{"Load"}</button>
            </div>
            <div>
                { "Stored Value: " } { (*stored_value).clone() }
            </div>
        </>
    }
}
