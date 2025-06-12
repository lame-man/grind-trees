use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

pub fn load_gtree_from_file(path: &str) -> GTree {
    let data = fs::read_to_string(path).expect("Failed to read JSON file");
    serde_json::from_str::<GTree>(&data).expect("Failed to parse JSON into GTree")
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub content: String,
    pub checked: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GNode {
    pub title: String,
    pub tasks: Vec<Task>,
    pub goals: Vec<String>,
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub parent: Option<usize>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GTree {
    pub title: String,
    pub nodes: Vec<GNode>,
    pub progress: f32,
}

pub struct Camera{
    dragging:bool,
    pub zoom: f32,
    pub offset: Vec2,
    last_mouse_pos: Vec2
}
impl Camera{
    pub fn new() -> Self {
        Self {
            offset: vec2(0.0, 0.0),
            zoom: 1.0,
            dragging: false,
            last_mouse_pos: vec2(0.0, 0.0),
        }
    }
    pub fn update(&mut self) {
        let mouse = vec2(mouse_position().0, mouse_position().1);

        // --- Dragging ---
        if is_mouse_button_pressed(MouseButton::Left) {
            self.dragging = true;
            self.last_mouse_pos = mouse;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            let delta = mouse - self.last_mouse_pos;
            self.offset += delta;
            self.last_mouse_pos = mouse;
        }

        // --- Zoom ---
        let scroll = mouse_wheel().1;
        if scroll != 0.0 {
            let zoom_factor = 1.1;
            let new_zoom = if scroll > 0.0 {
                self.zoom * zoom_factor
            } else {
                self.zoom / zoom_factor
            };

            // Zoom towards mouse position
            let before = (mouse - self.offset) / self.zoom;
            self.zoom = new_zoom.clamp(0.1, 10.0);
            let after = (mouse - self.offset) / self.zoom;
            self.offset += (after - before) * self.zoom;
        }
    }
}

pub fn calc_pos(original:Vec2, zoom:f32, camera_offset:Vec2)->Vec2{
    (original * zoom) + camera_offset
}

pub async fn draw_node(cam: &Camera, node: &GNode){  
    let circle_texture = load_texture("assets/circle.png").await.unwrap();
    let new_pos: Vec2 = calc_pos(vec2(node.x, node.y), cam.zoom, cam.offset);
    let size = node.r * cam.zoom;
    // Offset x and y so that the texture is centered at new_pos
    draw_texture_ex(
        &circle_texture,
        new_pos.x - size / 2.0,
        new_pos.y - size / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
    //draw_circle(new_pos.x, new_pos.y,node.r*cam.zoom, YELLOW);
}
