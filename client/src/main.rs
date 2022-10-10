// pub mod camera;
// pub mod selection;
// pub mod state;
// pub mod ui;

// use camera::CameraPlugin;
// use selection::SelectionPlugin;
// use state::StatePlugin;
// use ui::UiPlugin;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_renet::RenetClientPlugin;
use renet::{
    ClientAuthentication, RenetClient, RenetConnectionConfig, RenetError, NETCODE_USER_DATA_BYTES,
};
use std::{net::UdpSocket, time::SystemTime};
use store::{GameEvent, GameState};

// This id needs to be the same as the server is using
const PROTOCOL_ID: u64 = 1208;

fn main() {
    // Get username from stdin args
    let args = std::env::args().collect::<Vec<String>>();
    let username = &args[1];

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 480.0,
        height: 540.0,
        title: format!("GalacticWars <{}>", username),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::hex("282828").unwrap()))
    .add_plugins(DefaultPlugins)
    // Renet setup
    .add_plugin(RenetClientPlugin)
    .insert_resource(new_renet_client(&username).unwrap())
    .add_system(handle_renet_error)
    // Add game state and register GameEvent
    .insert_resource(GameState::default())
    .add_event::<GameEvent>()
    .add_startup_system(setup)
    .add_system(input)
    .add_system(update_waiting_text);

    // my own code
    // .add_startup_system(setup_level);
    // .add_plugin(MaterialPlugin::<PlanetMaterial>::default())

    app.run();
}

////////// COMPONENTS /////////////
#[derive(Component)]
struct UIRoot;

#[derive(Component)]
struct WaitingText;

////////// SETUP /////////////
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TicTacTussle is a 2D game
    // We need a 2d camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn board background
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, -30.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(480.0, 480.0)),
            ..default()
        },
        texture: asset_server.load("background.png").into(),
        ..default()
    });

    // Spawn pregame ui
    commands
        // A container that centers its children on the screen
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(UIRoot)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                    "Waiting for an opponent...",
                    TextStyle {
                        font: asset_server.load("Inconsolata.ttf"),
                        font_size: 24.0,
                        color: Color::hex("ebdbb2").unwrap(),
                    },
                ))
                .insert(WaitingText);
        });
}

/////////// UPDATE SYSTEMTS /////////////

fn update_waiting_text(mut text_query: Query<&mut Text, With<WaitingText>>, time: Res<Time>) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let num_dots = (time.time_since_startup().as_secs() % 3) + 1;
        text.sections[0].value = format!(
            "Waiting for an opponent{}{}",
            ".".repeat(num_dots as usize),
            // Pad with spaces to avoid text changing width and dancing all around
            " ".repeat(3 - num_dots as usize)
        );
    }
}

fn input(windows: Res<Windows>, input: Res<Input<MouseButton>>, game_state: Res<GameState>) {
    let window = windows.get_primary().unwrap();
    if let Some(mouse_position) = window.cursor_position() {
        // Determine if the index of the tile that the mouse is currently over
        // NOTE: This calculation assumes a fixed window size.
        // That's fine for now, but consider using the windows size instead.
        let x_tile: usize = (mouse_position.x / 160.0).floor() as usize;
        let y_tile: usize = (mouse_position.y / 160.0).floor() as usize;
        let tile = x_tile + y_tile * 3;

        // If mouse is outside of board we do nothing
        if 8 < tile {
            return;
        }

        // If left mouse button is pressed, send a place tile event to the server
        if input.just_pressed(MouseButton::Left) {
            info!("place piece at tile {:?}", tile);
        }
    }
}

//////////// RENET NETWORKING //////////////
// Creates a RenetClient that is already connected to a server.
// Returns an Err if connections fails
fn new_renet_client(username: &String) -> anyhow::Result<RenetClient> {
    let server_addr = "127.0.0.1:5000".parse()?;
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;

    // Place username in user data
    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too big");
    }
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let client = RenetClient::new(
        current_time,
        socket,
        client_id,
        RenetConnectionConfig::default(),
        ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: Some(user_data),
        },
    )?;

    Ok(client)
}

// If there's any network error we just panic
// Ie. Client has lost connection to server, if internet is gone or server shudown
fn handle_renet_error(mut renet_error: EventReader<RenetError>) {
    for err in renet_error.iter() {
        panic!("{}", err);
    }
}

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10., 1., 10.))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(5., 0.5, 5.));
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
