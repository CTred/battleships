pub mod components;
mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

pub const HEX_CONFIG_SIZE: f32 = 1.0;
pub const HEX_CONFIG_PADDING: f32 = 0.1;
pub const HEX_TOT_SIZE: f32 = HEX_CONFIG_SIZE + HEX_CONFIG_PADDING;

const CUBE_NEIGHBORS: [CubeCoords; 6] = [
    CubeCoords { q: 1, r: 0, s: -1 },
    CubeCoords { q: 1, r: -1, s: 0 },
    CubeCoords { q: 0, r: -1, s: 1 },
    CubeCoords { q: -1, r: 0, s: 1 },
    CubeCoords { q: -1, r: 1, s: 0 },
    CubeCoords { q: 0, r: 1, s: -1 },
];

const CUBE_DIAGONALS: [CubeCoords; 6] = [
    CubeCoords { q: 2, r: -1, s: -1 },
    CubeCoords { q: 1, r: -2, s: 1 },
    CubeCoords { q: -1, r: -1, s: 2 },
    CubeCoords { q: -2, r: 1, s: 1 },
    CubeCoords { q: -1, r: 2, s: -1 },
    CubeCoords { q: 1, r: 1, s: -2 },
];

pub struct HexPlugin;
impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HexMapTiles::default())
            .insert_resource(HexMapObjects::default())
            // TODO: CHECK IF HEXMAP RESOURCE IS ACTUALLY NECESSARY.
            .insert_resource(HexMap::new_from_axial(8, 1.0, 0.1))
            .insert_resource(MouseCubePos::default())
            // TODO: MOUSE CUBE POS NEED TO BE UPDATED FIRST
            .add_system(world_pos_to_cube_coords)
            .add_system(update_hover_hex)
            .add_system(hex_activate)
            .add_system(hex_draw_line)
            .add_startup_system(setup);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_config: Res<HexMap>,
    mut board_entities: ResMut<HexMapTiles>,
) {
    // Spawn board background

    // Spawn pregame ui

    // Spawn hexmap
    for hex in &board_config.hexes {
        let hex_pos = hex.world_pos();
        let entity = commands
            .spawn(MaterialMeshBundle {
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
