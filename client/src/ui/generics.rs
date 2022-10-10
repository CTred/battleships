use crate::assets::ImageAssets;
use bevy::prelude::{Res, World};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{Edge, Style},
        use_state, widget, EventType, OnEvent, WidgetProps,
    },
    widgets::{Element, Image, NinePatch, Text},
};

use super::styles::*;

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct ImageAndTextProps {
    pub image: u16,
    pub text: String,
}
#[widget]
pub fn ImageAndTextBox(props: ImageAndTextProps) {
    let p = props.clone();
    rsx! {
        <Element styles={Some(row())}>
            <Image styles={Some(image_styles())} handle={p.image} />
            <Text size={20.} content={p.text.to_owned()} />
        </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct SnakeButtonProps {
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
    #[prop_field(Children)]
    pub children: Option<kayak_ui::core::Children>,
}
#[widget]
pub fn SnakeButton(props: SnakeButtonProps) {
    let (dark_bg, light_bg) = context.query_world::<Res<ImageAssets>, _, _>(|assets| {
        (assets.bg_dark.clone(), assets.bg_light.clone())
    });
    let (dark_bg_img, light_bg_img) = context
        .get_global_mut::<World>()
        .map(|mut world| {
            let mut image_manager = world.get_resource_mut::<ImageManager>().unwrap();
            (image_manager.get(&dark_bg), image_manager.get(&light_bg))
        })
        .unwrap();

    // === State === //
    let initial_button_color = dark_bg_img;
    let (current_button_color, set_color, ..) = use_state!(initial_button_color);

    // EVENTS
    let cloned_current_button_color = current_button_color.clone();
    let parent_on_event = props.on_event.clone();
    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::MouseDown(..) => set_color(light_bg_img),
        EventType::MouseUp(..) => set_color(dark_bg_img),
        EventType::Click(..) => {
            match &parent_on_event {
                Some(v) => v.try_call(ctx, event),
                None => todo!(),
            };
        }
        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(24.0)}
            handle={current_button_color}
            styles={Some(button_style())}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}
