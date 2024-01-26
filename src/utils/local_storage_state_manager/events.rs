use crate::utils::local_storage_state_manager::local_storage_state::LocalStorageState;
use web_sys::{InputEvent, MouseEvent};
use yew::prelude::*;

/// Represents a union of different browser events.
///
/// This enum is used to abstract over different types of events (like input and mouse events)
/// so that they can be handled through a unified interface.
pub enum Event {
    /// Represents a browser input event.
    Input(InputEvent),
    /// Represents a browser mouse event.
    Mouse(MouseEvent),
}

/// Creates a closure that modifies the given `LocalStorageState` based on an `Event`.
///
/// This function is a higher-order function that takes an action and a state,
/// and returns a new closure. The closure takes an `Event` and applies the action
/// to the `LocalStorageState`, updating the state accordingly.
///
/// # Arguments
/// * `action` - A function that takes a mutable reference to `LocalStorageState` and an `Event`,
///   and performs some operations based on that event.
/// * `state` - A `UseStateHandle` for `LocalStorageState`, which is the current state
///   of a component in a Yew functional component.
///
/// # Returns
/// A closure that can be used as an event handler in Yew components. This closure takes
/// an `Event` and applies the specified `action` to the provided `state`.
///

pub fn make_callback(
    action: fn(&mut LocalStorageState, Event),
    state: UseStateHandle<LocalStorageState>,
) -> impl Fn(Event) {
    move |event: Event| {
        let mut new_state = (*state).clone();
        action(&mut new_state, event);
        state.set(new_state);
    }
}
