use yew::prelude::*;

use crate::utils::database::meditation_log::{
    calculate_meditation_stats::{Stats, StatsBuilder},
    read_write_meditation_data::read_meditation_data,
};

#[hook]
pub fn use_meditation_data() -> UseStateHandle<Stats> {
    let data_state: UseStateHandle<Stats> = use_state(|| Stats {
        ..StatsBuilder::new(vec![]).build()
    });
    let data_state_cl = data_state.clone();
    use_effect_with((), move |_| {
        let data = read_meditation_data();
        let stats = StatsBuilder::new(data).build();
        data_state_cl.set(stats);
    });

    data_state
}
