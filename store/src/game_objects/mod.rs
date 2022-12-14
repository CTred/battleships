pub mod components;
pub mod systems;

use crate::{
    map::components::world_pos_to_coordinates, GameEvent, GameStage, GameState, PlayerId, WhoAmI,
};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
// use serde::{Deserialize, Serialize};

pub use components::*;
use renet::RenetClient;

use crate::map::{
    components::{CubeCoords, Hexagon},
    HEX_CONFIG_PADDING, HEX_CONFIG_SIZE,
};

pub const SHIPS: [GameObject; 4] = [
    GameObject::Cruizer,
    GameObject::Ship,
    GameObject::Boat,
    GameObject::Boat,
];

#[derive(Resource)]
pub struct Garage(pub Vec<GameObject>);

pub struct GameObjectsPlugin;
impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::object_mouse_rotate)
            .add_system(systems::object_mouse_follow)
            .add_system(systems::object_mouse_hover)
            .add_system(systems::object_mouse_place_send)
            .add_system(systems::object_mouse_place_consume)
            .add_system_set(SystemSet::on_enter(GameStage::PreGame).with_system(populate_garage))
            .add_system_set(SystemSet::on_update(GameStage::PreGame).with_system(place_ships));
    }
}

fn populate_garage(mut commands: Commands) {
    let mut garage = Vec::new();
    for s in SHIPS {
        garage.push(s);
    }
    commands.insert_resource(Garage(garage));
}

fn place_ships(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut garage: ResMut<Garage>,
    who_am_i: Res<WhoAmI>,
    query: Query<Entity, With<MouseFollow>>,
    mut client: ResMut<RenetClient>,
) {
    if query.is_empty() {
        let obj_from_garage = garage.0.pop();
        match obj_from_garage {
            // if there are ships to place
            Some(obj) => {
                let ship = spawn_object(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &who_am_i.0,
                    &obj,
                    0,
                    Transform::from_xyz(0.0, 0.0, 2.0),
                    Color::BLUE,
                );
                commands.entity(ship).insert(MouseFollow);
            }
        }
    }
}

/// Spawns an object to world.
pub fn spawn_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    player_id: &PlayerId,
    game_object: &GameObject,
    angular_rot: i32,
    transform: Transform,
    base_color: Color,
) -> Entity {
    commands
        .spawn((
            ObjectBundle::new(game_object, angular_rot),
            MaterialMeshBundle {
                mesh: meshes.add(to_mesh(game_object)),
                material: materials.add(StandardMaterial {
                    base_color,
                    ..default()
                }),
                transform,
                ..default()
            },
        ))
        .id()
}

/// Generate a ['MaterialMeshBundle'] based on Hexagon coordinates and game object type.
pub fn to_mesh(game_object: &GameObject) -> Mesh {
    let hex_dimensions = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, None, 2.0);
    let triangle_top = [0.0, hex_dimensions.height * 0.25, 0.0];
    let upper_left = [-hex_dimensions.width / 4.0, 0.0, 0.0];
    let upper_right = [hex_dimensions.width / 4.0, 0.0, 0.0];
    let mut vectors = Vec::new();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    match game_object {
        GameObject::Boat => {
            let bottom_left = [
                -hex_dimensions.width / 4.0,
                -hex_dimensions.height * 1.2,
                0.0,
            ];
            let bottom_right = [
                hex_dimensions.width / 4.0,
                -hex_dimensions.height * 1.2,
                0.0,
            ];
            vectors.push(upper_left);
            vectors.push(bottom_left);
            vectors.push(bottom_right);
            vectors.push(upper_right);
            vectors.push(triangle_top);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 5]);
            mesh.set_indices(Some(Indices::U32(vec![0, 1, 3, 1, 2, 3, 0, 3, 4])));
        }
        GameObject::Ship => {
            let bottom_left = [
                -hex_dimensions.width / 4.0,
                -hex_dimensions.height * 1.2 * 1.7,
                0.0,
            ];
            let bottom_right = [
                hex_dimensions.width / 4.0,
                -hex_dimensions.height * 1.2 * 1.7,
                0.0,
            ];
            vectors.push(upper_left);
            vectors.push(bottom_left);
            vectors.push(bottom_right);
            vectors.push(upper_right);
            vectors.push(triangle_top);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 5]);
            mesh.set_indices(Some(Indices::U32(vec![0, 1, 3, 1, 2, 3, 0, 3, 4])));
        }
        GameObject::Cruizer => {
            let bottom_left = [
                -hex_dimensions.width / 4.0,
                -hex_dimensions.height * 1.2 * 2.5,
                0.0,
            ];
            let bottom_right = [
                hex_dimensions.width / 4.0,
                -hex_dimensions.height * 1.2 * 2.5,
                0.0,
            ];
            vectors.push(upper_left);
            vectors.push(bottom_left);
            vectors.push(bottom_right);
            vectors.push(upper_right);
            vectors.push(triangle_top);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 5]);
            mesh.set_indices(Some(Indices::U32(vec![0, 1, 3, 1, 2, 3, 0, 3, 4])));
        }
    }
    mesh
}

pub fn get_max_grid_rotation(game_object: &GameObject) -> i32 {
    match game_object {
        GameObject::Boat => 6 * 1,
        GameObject::Ship => 6 * 2,
        GameObject::Cruizer => 6 * 3,
    }
}
pub fn get_object_all_coords(
    game_object: &GameObject,
    rotation: i32,
    at: &CubeCoords,
) -> Vec<CubeCoords> {
    match game_object {
        GameObject::Boat => {
            let end_cube = hex_end_rotate(rotation, 2);
            line_coords(*at, end_cube + *at)
        }
        GameObject::Ship => {
            let end_cube = hex_end_rotate(rotation, 3);
            line_coords(*at, end_cube + *at)
        }
        GameObject::Cruizer => {
            let end_cube = hex_end_rotate(rotation, 4);
            line_coords(*at, end_cube + *at)
        }
    }
}

fn hex_end_rotate(rotation: i32, object_len: u32) -> CubeCoords {
    let max_valid_rotations = 6 * (object_len - 1);
    let mut u_rotation = rotation;
    if u_rotation < 0 {
        u_rotation = max_valid_rotations as i32 + u_rotation;
    }
    let coord_vector = build_coordinate_vector(object_len);
    let q = coord_vector[u_rotation as usize];
    let r =
        coord_vector[((u_rotation as u32 + 2 * (object_len - 1)) % max_valid_rotations) as usize];
    let s = -r - q;
    CubeCoords { q, r, s }
}

pub fn build_coordinate_vector(object_len: u32) -> Vec<i32> {
    let displacement = object_len - 1;
    let vector_len = 6 * displacement;
    let mut first_counter: i32 = 0;
    let mut direction = 1;
    let mut coord_pos = Vec::with_capacity(vector_len as usize);
    while coord_pos.len() < vector_len as usize {
        if first_counter.abs() < displacement as i32 {
            coord_pos.push(first_counter);
            first_counter += direction;
        }
        if first_counter.abs() >= displacement as i32 {
            for _ in 0..object_len {
                coord_pos.push((object_len - 1) as i32 * direction);
            }
            direction *= -1;
            first_counter += direction;
        }
    }
    coord_pos
}

pub fn line_coords(coord_origin: CubeCoords, coord_end: CubeCoords) -> Vec<CubeCoords> {
    let hex_origin = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(coord_origin), 0.0);
    let hex_end = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(coord_end), 0.0);
    let distance = (coord_end - coord_origin).magnitude();

    let points: Vec<Vec3> = (0..=distance)
        .map(|i| {
            lerp(
                hex_origin.world_pos(),
                hex_end.world_pos(),
                i as f32 / (distance) as f32,
            )
        })
        .collect();
    points
        .into_iter()
        .map(|pos| {
            world_pos_to_coordinates(
                HEX_CONFIG_SIZE + HEX_CONFIG_PADDING,
                Vec2::new(pos.x, pos.y),
            )
        })
        .collect()
}

fn lerp(start: Vec3, end: Vec3, distance: f32) -> Vec3 {
    start + (end - start) * distance
}
