use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub const STARTING_GAME_STATE: GameState = GameState::InGame;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Options,
    InGame,
    Pause,
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(STARTING_GAME_STATE)
            .add_system(stage_key_bindings.run_not_in_state(GameState::MainMenu))
            .add_enter_system(GameState::InGame, setup_game);
    }
}

fn stage_key_bindings(
    mut commands: Commands,
    kb_input: Res<Input<KeyCode>>,
    cur_state: Res<CurrentState<GameState>>,
) {
    if kb_input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
        dbg!("ESC");
    }
    if kb_input.just_pressed(KeyCode::Space) {
        match cur_state.0 {
            GameState::InGame => commands.insert_resource(NextState(GameState::Pause)),
            GameState::Pause => commands.insert_resource(NextState(GameState::InGame)),
            _ => {}
        }
        dbg!("SPACE");
    }
}

fn setup_game() {
    dbg!("reset_game!");
}
