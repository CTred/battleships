#[allow(unused_imports)]
use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Res, World};
use iyes_loopless::state::NextState;

use kayak_ui::bevy::ImageManager;
use kayak_ui::core::styles::Edge;
use kayak_ui::core::Binding;
use kayak_ui::core::{rsx, widget, Bound, EventType, OnEvent};
use kayak_ui::widgets::{If, NinePatch, Text};

use super::generics as gen;
use super::styles::*;
use crate::assets::ImageAssets;
use crate::state::GameState;

#[widget]
pub fn GameMenu() {
    // CSS
    let container_style = container_style()
        .with_style(bg_primary())
        .with_style(center());

    // RESOURCES
    let show_menus = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::MainMenu
    };

    let green_panel =
        context.query_world::<Res<ImageAssets>, _, _>(|assets| assets.bg_main.clone());
    let container = context
        .get_global_mut::<World>()
        .map(|mut world| {
            world
                .get_resource_mut::<ImageManager>()
                .unwrap()
                .get(&green_panel)
        })
        .unwrap();

    // EVENTS
    let on_click_new_game = OnEvent::new(|ctx, event| match event.event_type {
        EventType::Click(..) => {
            // dbg!("new game!");
            let mut world = ctx.get_global_mut::<World>().unwrap();
            world.insert_resource(NextState(GameState::InGame));
        }
        _ => {}
    });

    let on_click_settings = OnEvent::new(|_, event| match event.event_type {
        EventType::Click(..) => {
            dbg!("Settings");
        }
        _ => {}
    });

    let on_click_exit = OnEvent::new(|ctx, event| match event.event_type {
        EventType::Click(..) => ctx.query_world::<EventWriter<AppExit>, _, _>(|mut exit| {
            exit.send(AppExit);
        }),
        _ => {}
    });

    // RSX
    rsx! {
        <If condition={show_menus}>
            <NinePatch styles={Some(container_style)} border={Edge::all(15.0)} handle={container}
            >
                <gen::SnakeButton
                    on_event={Some(on_click_new_game)}
                >
                    <Text size={20.0} content={"New Game".to_string()} />
                </gen::SnakeButton>

                <gen::SnakeButton
                    on_event={Some(on_click_settings)}
                >
                    <Text size={20.0} content={"Settings".to_string()} />
                </gen::SnakeButton>

                <gen::SnakeButton
                    on_event={Some(on_click_exit)}
                >
                    <Text size={20.0} content={"Exit".to_string()} />
                </gen::SnakeButton>
            </NinePatch>
        </If>
    }
}
