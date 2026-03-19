use super::components::toast::Toast;
use crate::Gum;
use leptos::prelude::*;

pub struct PackConfigurationState {
    pub pack: RwSignal<Vec<String>>,
    pub editing_slot: RwSignal<Option<usize>>,
    pub preview: RwSignal<Option<Gum>>,
    pub filter_mode: RwSignal<String>,
    pub toasts: RwSignal<Vec<Toast>>,
}

pub fn create_pack_configuration_state() -> PackConfigurationState {
    PackConfigurationState {
        pack: RwSignal::new(vec![]),
        editing_slot: RwSignal::new(None),
        preview: RwSignal::new(None),
        filter_mode: RwSignal::new("classic".to_string()),
        toasts: RwSignal::new(vec![]),
    }
}
