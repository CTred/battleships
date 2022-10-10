use bevy::prelude::{Res, World};

use kayak_ui::bevy::ImageManager;
use kayak_ui::core::styles::{Corner, Edge, LayoutType};
use kayak_ui::core::{
    rsx,
    styles::{Style, StyleProp, Units},
    widget, Bound, WidgetProps,
};
use kayak_ui::core::{Binding, Color};
use kayak_ui::widgets::{Background, Element, If, Text};

use super::generics::*;
use super::styles::*;
use crate::assets::ImageAssets;
use crate::game;
use crate::state::GameState;

#[widget]
pub fn PauseMenu() {
    let show_pause = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::Pause
    };
    let pause_ribbon = Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),

        height: StyleProp::Value(Units::Percentage(40.0)),
        width: StyleProp::Value(Units::Percentage(100.0)),

        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::new(0.6, 0.6, 0.6, 0.4)),
        border_radius: StyleProp::Value(Corner::all(15.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    };

    // RSX
    rsx! {
        <If condition={show_pause}>
            <Background styles={Some(pause_ribbon)}>
                <Text size={20.0} content={"GAME PAUSED".to_string()} />
            </Background>
        </If>
    }
}

#[widget]
pub fn InGameUI() {
    let in_game = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::InGame
    };
    rsx! {
        <If condition={in_game}>
            <TopNavBar/>
            // <MultiplayerAndLog/>
            // <GroupsBar/>
            // <MiniMap/>
            // <ChatBar/>
        </If>
    }
}

#[widget]
pub fn TopNavBar() {
    let nav_bar = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Percentage(100.)),
        height: StyleProp::Value(Units::Pixels(40.)),
        ..Default::default()
    };

    let (fighter_h, trader_h, dreadn_h, planet_h) =
        context.query_world::<Res<ImageAssets>, _, _>(|assets| {
            (
                assets.figther_handle.clone(),
                assets.trader_handle.clone(),
                assets.dreadn_handle.clone(),
                assets.planet_handle.clone(),
            )
        });

    let (fighter_img, trader_img, dreadn_img, planet_img) = context
        .get_global_mut::<World>()
        .map(|mut world| {
            let mut img_manager = world.get_resource_mut::<ImageManager>().unwrap();
            (
                img_manager.get(&fighter_h),
                img_manager.get(&trader_h),
                img_manager.get(&dreadn_h),
                img_manager.get(&planet_h),
            )
        })
        .unwrap();
    rsx! {
        <Background styles={Some(nav_bar.with_style(center_top().with_style(bg_primary())))}>
            <Resources/>
            <ShipsAndPlanetsDetail fighter_img trader_img dreadn_img planet_img />
            <QuickMenu/>
        </Background>
    }
}

#[widget]
pub fn Resources() {
    let (r, g, b, a) = COLOR_TEXT;
    let text_color = Style {
        color: StyleProp::Value(Color::new(r / 256., g / 256., b / 256., a)),
        left: StyleProp::Value(Units::Pixels(10.)),
        ..Default::default()
    };
    rsx! {
        <Background styles={Some(center_left().with_style(row()))}>
            <Text size={40.0} content={"Cr$ 5000".to_string()} styles={Some(text_color)} />
        </Background>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct ShipsAndPlanetsProps {
    pub fighter_img: u16,
    pub trader_img: u16,
    pub dreadn_img: u16,
    pub planet_img: u16,
}

#[widget]
pub fn ShipsAndPlanetsDetail(props: ShipsAndPlanetsProps) {
    let ships_nav_bar = Style {
        col_between: StyleProp::Value(Units::Pixels(45.)),
        ..Default::default()
    };

    let fighters_deployed = {
        let fighter_count = context
            .query_world::<Res<Binding<game::resources::game_obj_res::FightersDeployed>>, _, _>(
                move |count| count.clone(),
            );
        context.bind(&fighter_count);
        fighter_count.get().0
    };
    let fighters_stored = {
        let fighter_count = context
            .query_world::<Res<Binding<game::resources::game_obj_res::FightersStored>>, _, _>(
                move |count| count.clone(),
            );
        context.bind(&fighter_count);
        fighter_count.get().0
    };
    let tot = fighters_deployed + fighters_stored;
    rsx! {
        <Background styles={Some(ships_nav_bar.with_style(center()).with_style(row()))}>
            // BUG: img+text does not reposition and text may overlap for large numbers
            <ImageAndTextBox image={props.fighter_img} text={tot.to_string()} />
            <ImageAndTextBox image={props.trader_img} text={"9".to_string()} />
            <ImageAndTextBox image={props.dreadn_img} text={"2".to_string()} />
            <ImageAndTextBox image={props.planet_img} text={"30".to_string()} />
        </Background>
    }
}

#[widget]
pub fn QuickMenu() {
    let quickmenu_style = Style {
        col_between: StyleProp::Value(Units::Pixels(15.)),
        ..Default::default()
    };
    rsx! {
        <Background styles={Some(quickmenu_style.with_style(center_right()).with_style(row()))}>
            <Text size={20.0} content={"A".to_string()} />
            <Text size={20.0} content={"N".to_string()} />
            <Text size={20.0} content={"S".to_string()} />
            <Text size={20.0} content={"M".to_string()} />
        </Background>
    }
}

#[widget]
pub fn IconAndText() {
    rsx! {
        <Element>
            <Text size={20.0} content={"5000".to_string()} />
        </Element>
    }
}
// #[widget]
// pub fn MultiplayerAndLog() {}

// #[widget]
// pub fn GroupsBar() {}

// #[widget]
// pub fn MiniMap() {}

// #[widget]
// pub fn ChatBar() {}
