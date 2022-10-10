use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::camera::MouseWorldPos;
use crate::game::utils::layers_util;

use super::components::*;
// TODO: selection hotkeys: to select all ships / all planets / all ships of specific type

pub fn update_box(
    mut commands: Commands,
    mut ev_select_box: EventReader<SelectMany>,
    query_selected: Query<Entity, With<Selected>>,
    query: Query<(Entity, &Transform), With<Selectable>>,
) {
    for ev in ev_select_box.iter() {
        for s_entity in query_selected.iter() {
            commands.entity(s_entity).remove::<Selected>();
        }
        if ev.bottom_left.distance(ev.top_right) < 1. {
            let mut min_entity = None;
            let mut min_dist: f32 = 10.0;
            for (e, transf) in query.iter() {
                let mouse_pos_3d =
                    layers_util::vec2_to_vec3(ev.bottom_left, layers_util::Layers::Planets);
                let obj_dist = mouse_pos_3d.distance_squared(transf.translation);
                if obj_dist <= min_dist {
                    min_dist = obj_dist;
                    min_entity = Some(e);
                }
            }
            if let Some(e) = min_entity {
                commands.entity(e).insert(Selected);
            }
        } else {
            // TODO: split selection between ships and planets.
            for (e, transf) in query.iter() {
                if transf.translation.x >= ev.bottom_left.x
                    && transf.translation.x <= ev.top_right.x
                    && transf.translation.y >= ev.bottom_left.y
                    && transf.translation.y <= ev.top_right.y
                {
                    commands.entity(e).insert(Selected);
                }
            }
        }
    }
}

pub fn box_select(
    mut commands: Commands,
    ms_input: Res<Input<MouseButton>>,
    ms_pos: Res<MouseWorldPos>,
    selection_box: Query<Entity, With<SelectionBox>>,
    mut is_selecting_res: ResMut<IsSelecting>,
    mut ev_select_writer: EventWriter<SelectMany>,
) {
    if ms_input.just_pressed(MouseButton::Left) {
        is_selecting_res.is_selecting = true;
        is_selecting_res.mouse_enter = Some(ms_pos.0);
    }
    if ms_input.just_released(MouseButton::Left) {
        match selection_box.get_single() {
            Ok(e) => {
                commands.entity(e).despawn();
                if let Some(mouse_enter) = is_selecting_res.mouse_enter {
                    let bottom_left =
                        Vec2::new(mouse_enter.x.min(ms_pos.0.x), mouse_enter.y.min(ms_pos.0.y));
                    let top_right =
                        Vec2::new(mouse_enter.x.max(ms_pos.0.x), mouse_enter.y.max(ms_pos.0.y));
                    ev_select_writer.send(SelectMany {
                        bottom_left,
                        top_right,
                    });
                }
            }
            Err(_) => {}
        }
        is_selecting_res.is_selecting = false;
        is_selecting_res.mouse_enter = None;
    }
}

pub fn draw_box_select(
    mut commands: Commands,
    is_selecting_res: Res<IsSelecting>,
    ms_pos: Res<MouseWorldPos>,
    mut query: Query<(&mut Handle<Mesh>, &mut Transform), With<SelectionBox>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if is_selecting_res.is_selecting {
        if let Some(ms_enter) = is_selecting_res.mouse_enter {
            match query.get_single_mut() {
                Ok((mut box_mesh, mut transform)) => {
                    *box_mesh = meshes.add(box_select_mesh(ms_enter, ms_pos.0));
                    transform.translation =
                        layers_util::vec2_to_vec3(ms_enter, layers_util::Layers::BoxSelect);
                    // *box_material = materials.add(SelectBoxMaterial {
                    //     color_outline: Color::LIME_GREEN,
                    //     bottom_left: ms_enter,
                    //     top_right: ms_pos.0,
                    // })
                }
                Err(error) => {
                    match error {
                        QuerySingleError::NoEntities(_) => {
                            // let quad_handle = meshes.add(box_select_mesh(ms_enter, ms_pos.0));
                            let quad_handle = meshes.add(box_select_mesh(ms_enter, ms_pos.0));
                            let material_handle = materials.add(StandardMaterial {
                                base_color: Color::Rgba {
                                    red: 0.2,
                                    green: 0.8,
                                    blue: 0.2,
                                    alpha: 0.3,
                                },
                                unlit: true,
                                alpha_mode: AlphaMode::Blend,
                                ..default()
                            });
                            commands
                                .spawn_bundle(MaterialMeshBundle {
                                    mesh: quad_handle.clone(),
                                    material: material_handle,
                                    transform: Transform {
                                        translation: layers_util::vec2_to_vec3(
                                            ms_pos.0,
                                            layers_util::Layers::BoxSelect,
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(SelectionBox);
                        }
                        QuerySingleError::MultipleEntities(_) => {
                            panic!("Expected one entity at most")
                        }
                    };
                }
            }
        }
    }
}

pub fn box_select_mesh(screen_pos_origin: Vec2, screen_pos_target: Vec2) -> Mesh {
    let start = Vec2::ZERO;
    let end = screen_pos_target - screen_pos_origin;

    let top_left = Vec2::new(start.x.min(end.x), start.y.max(end.y));
    let bottom_right = Vec2::new(start.x.max(end.x), start.y.min(end.y));
    let bottom_left = Vec2::new(start.x.min(end.x), start.y.min(end.y));
    let top_right = Vec2::new(start.x.max(end.x), start.y.max(end.y));
    let z_value = layers_util::get_z(layers_util::Layers::BoxSelect);

    // points are (vec3[position], vec2[uvs])
    // CHECK: UVs are random values
    let points = vec![
        ([bottom_left.x, bottom_left.y, z_value], [0.0, 1.0]),
        ([bottom_right.x, bottom_right.y, z_value], [1.0, 0.0]),
        ([top_left.x, top_left.y, z_value], [0.0, 1.0]),
        ([top_right.x, top_right.y, z_value], [1.0, 1.0]),
    ];
    let mut vertices = Vec::with_capacity(points.len());
    let mut uvs = Vec::with_capacity(points.len());
    let normals = vec![[0.0, 0.0, 1.0]; points.len()];

    for (position, uv) in points.iter() {
        vertices.push(*position);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 1, 3, 2])));
    mesh
}
