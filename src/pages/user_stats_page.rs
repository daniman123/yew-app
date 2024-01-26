use web_sys::{InputEvent, MouseEvent};
use yew::prelude::*;

use crate::utils::hooks::local_storage_state::{
    event_handlers::{on_key_change, on_load, on_save, on_value_change},
    events::{make_callback, Event},
    local_storage_state::LocalStorageState,
};

#[function_component]
pub fn UserStats() -> Html {
    let state = use_state(LocalStorageState::new);

    let on_key_change_cb = {
        let callback = make_callback(on_key_change, state.clone());
        Callback::from(move |e: InputEvent| callback(Event::Input(e)))
    };

    let on_value_change_cb = {
        let callback = make_callback(on_value_change, state.clone());
        Callback::from(move |e: InputEvent| callback(Event::Input(e)))
    };

    let on_save_cb = {
        let callback = make_callback(on_save, state.clone());
        Callback::from(move |e: MouseEvent| callback(Event::Mouse(e)))
    };

    let on_load_cb = {
        let callback = make_callback(on_load, state.clone());
        Callback::from(move |e: MouseEvent| callback(Event::Mouse(e)))
    };

    html! {
        <>
            <div>
                <input placeholder="Key" oninput={on_key_change_cb} value={state.input_key.clone()} />
                <input placeholder="Value" oninput={on_value_change_cb} value={state.input_value.clone()} />
                <button onclick={on_save_cb}>{"Save"}</button>
                <button onclick={on_load_cb}>{"Load"}</button>
            </div>
            <div>
                { "Stored Value: " } { state.stored_value.clone() }
            </div>
        </>
    }
}
