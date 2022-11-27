use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Component)]
pub struct UiCamera;
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(startup);
    }
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn(UICameraBundle::new());
    let mut widget_context = KayakRootContext::new();
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    }
    commands.insert_resource(widget_context);
}

fn render_ui(windows: Res<Windows>) {
    // global variables
    let heigth = windows.primary().height();
    let width = windows.primary().width();

    // UI hierarchy
    let main_ui = Widget::default();
    main_ui.call(heigth, width);

    // traverse objects
}

#[derive(Default)]
pub struct Widget {
    pub parent: Option<f32>,
    pub children: Vec<Widget>,
    pub draw: Option<Draw>,
}
impl Widget {
    pub fn call(&self, heigth: f32, width: f32) {
        // draw object
        if let Some(d) = &self.draw {
            d.to_mesh();
        }
        // do the same for children objects
        for child in self.children.iter() {
            child.call(heigth, width);
        }
    }
}

pub struct Draw {
    pub location: f32,
    pub width: Option<f32>,
    pub heigth: Option<f32>,
}
impl Draw {
    pub fn to_mesh(&self) {}
}
