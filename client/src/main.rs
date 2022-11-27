use bevy::{prelude::*, window::PresentMode};
use bevy_renet::{run_if_client_connected, RenetClientPlugin};
use renet::{
    ClientAuthentication, RenetClient, RenetConnectionConfig, RenetError, NETCODE_USER_DATA_BYTES,
};
use std::{iter::Enumerate, net::UdpSocket, time::SystemTime};
use store::{
    camera::{CameraPlugin, MouseWorldPos},
    map::{
        components::{CubeCoords, Hexagon, MouseCubePos},
        HexPlugin, HEX_CONFIG_PADDING, HEX_CONFIG_SIZE,
    },
    ships::Ship,
    GameEvent, GameState,
};

use ui::UiPlugin;
// This id needs to be the same as the server is using
const PROTOCOL_ID: u64 = 1208;

fn main() {
    // Get username from stdin args
    let args = std::env::args().collect::<Vec<String>>();
    let username = &args[1];

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "BattleGrounds!".to_string(),
            width: 500.,
            height: 300.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }))
    .insert_resource(ClearColor(Color::hex("282828").unwrap()))
    // Renet setup
    .add_plugin(RenetClientPlugin::default())
    .insert_resource(new_renet_client(&username).unwrap())
    .add_system(handle_renet_error)
    // Add game state and register GameEvent
    .insert_resource(GameState::default())
    .add_event::<GameEvent>()
    // my own code
    .add_startup_system(spawns)
    .add_system(input)
    .add_plugin(HexPlugin)
    .add_plugin(UiPlugin)
    .add_system(update_board)
    .add_system_to_stage(
        CoreStage::PostUpdate,
        // Renet exposes a nice run criteria
        // that can be used to make sure that this system only runs when connected
        receive_events_from_server.with_run_criteria(run_if_client_connected),
    );

    app.add_plugin(CameraPlugin);
    // .add_startup_system(setup_level);
    // .add_plugin(MaterialPlugin::<PlanetMaterial>::default())

    app.run();
}

////////// COMPONENTS /////////////

////////// SETUP /////////////
fn spawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hex = Hexagon::new(
        HEX_CONFIG_SIZE,
        HEX_CONFIG_PADDING,
        Some(CubeCoords { q: 0, r: 0, s: 0 }),
        1.0,
    );

    let mut ship = Ship::new(
        store::ships::ShipType::Light,
        1_u64,
        CubeCoords { q: 0, r: 0, s: 0 },
        CubeCoords { q: 0, r: -1, s: 1 },
    );
    let pos = ship.world_pos();
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(ship.to_mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::GOLD,
            ..default()
        }),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        ..default()
    });

    let pos = ship.world_pos();
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(ship.to_mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::GOLD,
            ..default()
        }),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        ..default()
    });
}

/////////// UPDATE SYSTEMTS /////////////

fn input(
    input: Res<Input<MouseButton>>,
    kb_input: Res<Input<KeyCode>>,
    ms_coord_pos: Res<MouseCubePos>,
    game_state: Res<GameState>,
    mut client: ResMut<RenetClient>,
) {
    // If left mouse button is pressed, send mouse world pos
    if input.just_pressed(MouseButton::Left) {
        // We only want to handle inputs once we are ingame
        match game_state.stage {
            store::GameStage::PreGame => {
                let event = GameEvent::ShipPlaced {
                    player_id: client.client_id(),
                    at: ms_coord_pos.0,
                    rotation: 3,
                };
                client.send_message(0, bincode::serialize(&event).unwrap());
            }
            store::GameStage::InGame => {
                let event = GameEvent::ShipMove {
                    player_id: client.client_id(),
                    at: ms_coord_pos.0,
                };
                client.send_message(0, bincode::serialize(&event).unwrap());
            }
            _ => {
                return;
            }
        };
    }
    if kb_input.just_pressed(KeyCode::Space) {
        info!("game_state: {:?}", game_state);
    }
}

fn update_board(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut game_events: EventReader<GameEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in game_events.iter() {
        match event {
            GameEvent::ShipMove { player_id, at } => {
                info!("{:?} moved to {:?}", player_id, at);
            }
            GameEvent::ShipPlaced {
                player_id,
                at,
                rotation: _,
            } => {
                let ship = Ship::new(store::ships::ShipType::Light, 1_u64, *at, *at);
                let pos = ship.world_pos();
                commands.spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(ship.to_mesh()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::GOLD,
                        ..default()
                    }),
                    transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                    ..default()
                });
                info!("{:?} ship placed", at);
                // place_ship(&mut commands, at);
            }
            _ => {}
        }
    }
}

// fn place_ship(commands: &mut Commands, at: CubeCoords) {
//     let mut ship = Ship::new(store::ships::ShipType::Light, 1_u64, Some(at), &hex);
//     let pos = ship.world_pos();
//     commands.spawn_bundle(MaterialMeshBundle {
//         mesh: meshes.add(ship.to_mesh()),
//         material: materials.add(StandardMaterial {
//             base_color: Color::RED,
//             ..default()
//         }),
//         transform: Transform::from_xyz(pos.x, pos.y, pos.z),
//         ..default()
//     });
// }

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

fn receive_events_from_server(
    mut client: ResMut<RenetClient>,
    mut game_state: ResMut<GameState>,
    mut game_events: EventWriter<GameEvent>,
) {
    while let Some(message) = client.receive_message(0) {
        // Whenever the server sends a message we know it must be a game event
        let event: GameEvent = bincode::deserialize(&message).unwrap();
        trace!("{:#?}", event);

        // We trust the server, no need to validade events
        game_state.consume(&event);

        // Send the event into the bevy event system so systems can react to it
        game_events.send(event);
    }
}
