use bevy::prelude::*;
use bevy_renet::{run_if_client_connected, RenetClientPlugin};
use renet::{
    ClientAuthentication, RenetClient, RenetConnectionConfig, RenetError, NETCODE_USER_DATA_BYTES,
};
use std::{net::UdpSocket, time::SystemTime};
use store::{
    camera::{CameraPlugin, MouseWorldPos},
    map::{Hex, HexHover, HexMap, HexMapEntities, HexStatus, Hexagon},
    GameEvent, GameState,
};

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
        title: format!("BattleGrounds <{}>", username),
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
    // my own code
    .insert_resource(store::map::HexMapEntities::default())
    .insert_resource(store::map::HexMap::new_from_axial(8, 1.0, 0.1))
    .add_startup_system(setup)
    .add_system(input)
    .add_system(update_board)
    .add_system(update_hover_hex)
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
#[derive(Component)]
struct WaitingText;

type TileIndex = usize;
#[derive(Component)]
struct HoverDot(pub TileIndex);

////////// SETUP /////////////
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_config: Res<HexMap>,
    mut board_entities: ResMut<HexMapEntities>,
) {
    // Spawn board background

    // Spawn pregame ui

    // Spawn hexmap
    for hex in &board_config.hexes {
        let hex_pos = hex.world_pos();
        let entity = commands
            .spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(hex.to_mesh()),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.67, 0.67, 0.67),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(hex_pos.x, hex_pos.y, hex_pos.z),
                ..default()
            })
            .insert(Hex(HexStatus::Cold))
            .id();
        board_entities.0.insert(hex.coords.unwrap(), entity);
    }
}

/////////// UPDATE SYSTEMTS /////////////

fn input(
    input: Res<Input<MouseButton>>,
    ms_pos: Res<MouseWorldPos>,
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
                    at: ms_pos.0,
                };
                client.send_message(0, bincode::serialize(&event).unwrap());
            }
            store::GameStage::InGame => {
                let event = GameEvent::ShipMove {
                    player_id: client.client_id(),
                    at: ms_pos.0,
                };
                client.send_message(0, bincode::serialize(&event).unwrap());
            }
            _ => {
                return;
            }
        };
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
            GameEvent::ShipPlaced { player_id: _, at } => {
                info!("{:?} ship placed", at);
            }
            _ => {}
        }
    }
}

fn update_hover_hex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ms_pos: Res<MouseWorldPos>,
    board_config: Res<HexMap>,
    hex_board: Res<HexMapEntities>,
    mut query: Query<(&mut Transform, &mut Visibility), With<HexHover>>,
) {
    let cur_coord = board_config.world_pos_to_coordinates(ms_pos.0);
    let is_entity = match hex_board.0.get(&cur_coord) {
        Some(_) => true,
        None => false,
    };
    let hex_config = &board_config.hexes[0];
    let hex = Hexagon::new(hex_config.size, hex_config.padding, Some(cur_coord), 1.0);
    let hex_pos = hex.world_pos();

    match query.get_single_mut() {
        Ok((mut transf, mut vis)) => {
            transf.translation = Vec3::new(hex_pos.x, hex_pos.y, hex_pos.z);
            vis.is_visible = is_entity;
        }
        Err(query_error) => match query_error {
            bevy::ecs::query::QuerySingleError::NoEntities(_) => {
                commands
                    .spawn_bundle(MaterialMeshBundle {
                        mesh: meshes.add(hex.to_mesh()),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.87, 0.87, 0.87),
                            unlit: true,
                            ..default()
                        }),
                        transform: Transform::from_xyz(hex_pos.x, hex_pos.y, hex_pos.z),
                        visibility: Visibility {
                            is_visible: is_entity,
                        },
                        ..default()
                    })
                    .insert(HexHover);
            }
            bevy::ecs::query::QuerySingleError::MultipleEntities(_) => {
                panic!("expected one or no entity")
            }
        },
    }
}

fn hex_to_color(
    hex: &Hex,
    handle: &Handle<StandardMaterial>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut material = materials.get_mut(&handle).unwrap();
    match hex.0 {
        HexStatus::Cold => {
            material.base_color = Color::rgb(0.67, 0.67, 0.67);
        }
        HexStatus::Hot => {
            material.base_color = Color::rgb(0.87, 0.87, 0.87);
        }
        HexStatus::Selected => {
            material.base_color = Color::rgb(0.20, 0.90, 0.20);
        }
        HexStatus::Damage => {
            material.base_color = Color::rgb(0.9, 0.1, 0.1);
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
