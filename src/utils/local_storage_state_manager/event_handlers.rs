use crate::utils::local_storage_state_manager::events::Event;
use crate::utils::local_storage_state_manager::local_storage_state::LocalStorageState;

/// Updates the `input_key` of the `LocalStorageState` based on an input event.
///
/// This function is designed to be used as an event handler for input events
/// related to the `input_key` of `LocalStorageState`. It extracts the value from
/// the input event and updates the `input_key` state.
///
/// # Arguments
/// * `state` - A mutable reference to the `LocalStorageState`.
/// * `event` - The `Event` enum, expected to be `Event::Input`.
pub fn on_key_change(state: &mut LocalStorageState, event: Event) {
    if let Event::Input(e) = event {
        state.on_key_change(e);
    }
}

/// Updates the `input_value` of the `LocalStorageState` based on an input event.
///
/// This function is intended to handle input events related to the `input_value`.
/// It extracts the value from the event and updates the `input_value` state.
///
/// # Arguments
/// * `state` - A mutable reference to the `LocalStorageState`.
/// * `event` - The `Event` enum, expected to be `Event::Input`.
pub fn on_value_change(state: &mut LocalStorageState, event: Event) {
    if let Event::Input(e) = event {
        state.on_value_change(e);
    }
}

/// Triggers the save operation on `LocalStorageState` based on a mouse event.
///
/// This function is intended to handle mouse events (like button clicks) to
/// trigger saving the current state to local storage. It calls the `on_save`
/// method of `LocalStorageState`.
///
/// # Arguments
/// * `state` - A mutable reference to the `LocalStorageState`.
/// * `event` - The `Event` enum, expected to be `Event::Mouse`.
pub fn on_save(state: &mut LocalStorageState, event: Event) {
    if let Event::Mouse(_) = event {
        state.on_save();
    }
}

/// Triggers the load operation from local storage into `LocalStorageState` based on a mouse event.
///
/// This function is designed to respond to mouse events (like button clicks) to
/// load data from local storage. It updates the `LocalStorageState` with the
/// loaded value using the `on_load` method.
///
/// # Arguments
/// * `state` - A mutable reference to the `LocalStorageState`.
/// * `event` - The `Event` enum, expected to be `Event::Mouse`.
pub fn on_load(state: &mut LocalStorageState, event: Event) {
    if let Event::Mouse(_) = event {
        state.on_load();
    }
}
