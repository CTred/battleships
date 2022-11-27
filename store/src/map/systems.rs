use super::{
    components::{world_pos_to_coordinates, CubeCoords, SpawnHover},
    Hex, HexHover, HexMapEntities, HexStatus, Hexagon, MouseCubePos, HEX_CONFIG_PADDING,
    HEX_CONFIG_SIZE, HEX_TOT_SIZE,
};
use crate::{camera::MouseWorldPos, ships::components::Ship};
use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn world_pos_to_cube_coords(
    ms_pos: Res<MouseWorldPos>,
    mut ms_coord_pos: ResMut<MouseCubePos>,
) {
    ms_coord_pos.0 = world_pos_to_coordinates(HEX_TOT_SIZE, ms_pos.0);
}

pub fn update_hover_hex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ms_coord: Res<MouseCubePos>,
    hex_board: Res<HexMapEntities>,
    mut query: Query<(&mut Transform, &mut Visibility), With<HexHover>>,
) {
    let is_entity = match hex_board.0.get(&ms_coord.0) {
        Some(_) => true,
        None => false,
    };
    let hex = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(ms_coord.0), 1.0);
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
                            base_color: Color::rgba(0.87, 0.87, 0.87, 0.5),
                            unlit: true,
                            alpha_mode: AlphaMode::Blend,
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

// pub fn hover_rotate(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     ms_coord: Res<MouseCubePos>,
//     hex_board: Res<HexMapEntities>,
//     mut query: Query<(&mut Ship, &mut Transform), With<SpawnHover>>,
//     mut scroll_ev: EventReader<MouseWheel>,
// ) {
//     use crate::ships::ShipType::*;
//     for ev in scroll_ev.iter() {
//         for (mut ship, mut transf) in &mut query {
//             match ship.ship_type {
//                 Light => {
//                     for (i, mut hex) in ship.hexes.iter_mut().enumerate() {
//                         hex.coords = Some(hex.coords.unwrap() + Hexagon::RIGHT);
//                     }
//                 }
//             }
//         }
//     }
// }

pub fn hex_activate(
    ms_input: Res<Input<MouseButton>>,
    ms_coord: Res<MouseCubePos>,
    hex_board: Res<HexMapEntities>,
    mut query: Query<(&mut Hex, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if ms_input.just_pressed(MouseButton::Left) {
        if let Some(entity) = hex_board.0.get(&ms_coord.0) {
            if let Ok((mut hex, handle)) = query.get_mut(*entity) {
                update_hex_status(&mut hex);
                hex_to_color(&hex, handle, &mut materials);
            }
        }
    }
}

pub fn hex_draw_line(
    ms_input: Res<Input<MouseButton>>,
    ms_coord: Res<MouseCubePos>,
    hex_board: Res<HexMapEntities>,
    mut query: Query<(&mut Hex, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if ms_input.just_pressed(MouseButton::Right) {
        let hex_origin = Hexagon::new(
            HEX_CONFIG_SIZE,
            HEX_CONFIG_PADDING,
            Some(CubeCoords::ZERO),
            0.0,
        );
        let hex_end = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(ms_coord.0), 0.0);
        let distance = ms_coord.0.magnitude(); // NOTE: distance to coord 0,0,0
        let points: Vec<Vec3> = (0..=distance)
            .map(|i| {
                lerp(
                    hex_origin.world_pos(),
                    hex_end.world_pos(),
                    i as f32 / (distance) as f32,
                )
            })
            .collect();
        let entities: Vec<Option<&Entity>> = points
            .into_iter()
            .map(|pos| {
                let coords = world_pos_to_coordinates(
                    HEX_CONFIG_SIZE + HEX_CONFIG_PADDING,
                    Vec2::new(pos.x, pos.y),
                );
                hex_board.0.get(&coords)
            })
            .collect();
        for opt_entity in entities {
            if let Some(entity) = opt_entity {
                if let Ok((mut hex, handle)) = query.get_mut(*entity) {
                    update_hex_status(&mut hex);
                    hex_to_color(&hex, handle, &mut materials);
                }
            }
        }
    }
}

fn update_hex_status(hex: &mut Hex) {
    match hex.0 {
        HexStatus::Cold => hex.0 = HexStatus::Selected,
        HexStatus::Selected => hex.0 = HexStatus::Cold,
        _ => {}
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
        // HexStatus::Hot => {
        //     material.base_color = Color::rgb(0.87, 0.87, 0.87);
        // }
        HexStatus::Selected => {
            material.base_color = Color::rgb(0.20, 0.90, 0.20);
        }
        HexStatus::Damage => {
            material.base_color = Color::rgb(0.9, 0.1, 0.1);
        }
    }
}

fn lerp(start: Vec3, end: Vec3, distance: f32) -> Vec3 {
    start + (end - start) * distance
}
