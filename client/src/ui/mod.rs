mod generics;
mod ingame_ui;
mod menu_ui;
mod styles;

use bevy::prelude::{App, AssetServer, Commands, Plugin, Res, ResMut};
use iyes_loopless::state::CurrentState;

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{bind, Binding};
use kayak_ui::core::{render, MutableBound};
use kayak_ui::widgets::App as KApp;

use crate::game::resources;
use crate::state::{self, GameState};
use ingame_ui::*;
use menu_ui::*;

fn ui_startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    // mut image_manager: ResMut<ImageManager>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));

    let context = BevyContext::new(|context| {
        render! {
            <KApp>
                <GameMenu/>
                <PauseMenu/>
                <InGameUI/>
            </KApp>
        }
    });

    commands.insert_resource(context);
}

pub fn bind_gamestate(state: Res<CurrentState<GameState>>, binding: Res<Binding<GameState>>) {
    if state.is_changed() {
        binding.set(state.0.clone());
    }
}
pub fn bind_fighter_deployed(
    state: Res<resources::game_obj_res::FightersDeployed>,
    binding: Res<Binding<resources::game_obj_res::FightersDeployed>>,
) {
    if state.is_changed() {
        binding.set(state.clone());
    }
}
pub fn bind_fighter_stored(
    state: Res<resources::game_obj_res::FightersStored>,
    binding: Res<Binding<resources::game_obj_res::FightersStored>>,
) {
    if state.is_changed() {
        binding.set(state.clone());
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyKayakUIPlugin)
            .insert_resource(bind(state::STARTING_GAME_STATE))
            .insert_resource(bind(resources::game_obj_res::FightersDeployed(0)))
            .insert_resource(bind(resources::game_obj_res::FightersStored(0)))
            .insert_resource(bind(resources::game_obj_res::TotalTraders(0)))
            .insert_resource(bind(resources::game_obj_res::TotalDreadnoughts(0)))
            .insert_resource(bind(resources::game_obj_res::TotalPlanets(0)))
            .add_startup_system(ui_startup)
            .add_system(bind_gamestate)
            .add_system(bind_fighter_deployed)
            .add_system(bind_fighter_stored);
    }
}
