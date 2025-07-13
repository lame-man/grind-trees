use core::skill_tree::GTree;

use macroquad::prelude::*;

pub const DEFAULT_PAN_SPEED: f32 = 2.;
pub const DEFAULT_ZOOM_SPEED: f32 = 0.05;

pub struct AppState {
    pub pan: Vec2,
    pub zoom: f32,
    pub mouse_pos: (f32, f32),
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub menu_on: bool,
    pub load: bool,
    pub save: bool,
    pub quit: bool,
    pub file: Option<String>,
    pub skill_tree: Option<GTree>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            pan: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            mouse_pos: (0.0, 0.0),
            pan_speed: DEFAULT_PAN_SPEED,
            zoom_speed: DEFAULT_ZOOM_SPEED,
            menu_on: false,
            load: false,
            save: false,
            quit: false,
            file: None,
            skill_tree: None,
        }
    }
}
