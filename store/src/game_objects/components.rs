use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct MouseFollow;

#[derive(Component)]
pub struct ObjectHover;

#[derive(Component)]
pub struct GridMaxRotation(pub u8);

#[derive(Component)]
pub struct AngularRot(pub i32);

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Component)]
pub enum GameObject {
    Boat,
    Ship,
    Cruizer,
}

#[derive(Bundle)]
pub struct ObjectBundle {
    pub game_object: GameObject,
    grid_max_rotation: GridMaxRotation,
    pub angular_rot: AngularRot,
}

impl ObjectBundle {
    pub fn new(game_object: &GameObject, angular_rot: i32) -> Self {
        let grid_max_rotation = match game_object {
            GameObject::Boat => 6 * 1,
            GameObject::Ship => 6 * 2,
            GameObject::Cruizer => 6 * 3,
        };
        Self {
            game_object: game_object.clone(),
            grid_max_rotation: GridMaxRotation(grid_max_rotation),
            angular_rot: AngularRot(angular_rot),
        }
    }
}
