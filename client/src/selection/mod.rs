use crate::state::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
mod systems;
use systems::*;
pub mod components;
use components::*;

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IsSelecting {
            is_selecting: false,
            mouse_enter: None,
        })
        .add_event::<SelectMany>()
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(box_select)
                .with_system(update_box)
                .with_system(draw_box_select)
                .into(),
        );
    }
}
