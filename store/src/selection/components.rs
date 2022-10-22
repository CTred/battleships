use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy)]
pub struct Selected;

#[derive(Component, Clone, Copy)]
pub struct Selectable;

#[derive(Component, Clone, Copy)]
pub struct SelectionBox;

#[derive(Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IsSelecting {
    pub is_selecting: bool,
    pub mouse_enter: Option<Vec2>,
}

#[derive(Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SelectQuad {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
}
