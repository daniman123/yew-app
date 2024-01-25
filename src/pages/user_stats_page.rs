use web_sys::InputEvent;
use yew::prelude::*;

use crate::utils::hooks::local_storage_state::LocalStorageState;

#[function_component]
pub fn UserStats() -> Html {
    let state = use_state(LocalStorageState::new);

    let on_key_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_state = (*state).clone();
            new_state.on_key_change(e);
            state.set(new_state);
        })
    };

    let on_value_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_state = (*state).clone();
            new_state.on_value_change(e);
            state.set(new_state);
        })
    };

    let on_save = {
        let state = state.clone();
        Callback::from(move |_| {
            let new_state = (*state).clone();
            new_state.on_save();
            state.set(new_state);
        })
    };

    let on_load = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            new_state.on_load();
            state.set(new_state);
        })
    };

    html! {
        <>
            <div>
                <input placeholder="Key" oninput={on_key_change} value={state.input_key.clone()} />
                <input placeholder="Value" oninput={on_value_change} value={state.input_value.clone()} />
                <button onclick={on_save}>{"Save"}</button>
                <button onclick={on_load}>{"Load"}</button>
            </div>
            <div>
                { "Stored Value: " } { state.stored_value.clone() }
            </div>
        </>
    }
}
