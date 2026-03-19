use leptos::prelude::*;

pub struct RunState {
    pub pack: RwSignal<Vec<String>>,
    pub used_slots: RwSignal<Vec<usize>>,
    pub cycle_count: RwSignal<i32>,
}

pub fn create_run_state() -> RunState {
    RunState {
        pack: RwSignal::new(vec![]),
        used_slots: RwSignal::new(vec![]),
        cycle_count: RwSignal::new(0),
    }
}