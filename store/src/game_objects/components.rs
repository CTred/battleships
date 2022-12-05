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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Component)]
pub enum GameObject {
    Boat,
    Ship,
}
