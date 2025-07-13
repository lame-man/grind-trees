use macroquad::prelude::*;
use crate::gtree::{GNode, GTree, Task};

pub struct Camera{
    pub zoom: f32,
    pub offset: (f32, f32),
    pub mouse_position: (f32, f32),
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            zoom: 1.0,
            offset: (0.0, 0.0),
            mouse_position:  (0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.arrow_pan();
        self.mouse_pan();
        self.scroll_zoom();
    }
    fn arrow_pan(&mut self){
        if is_key_down(KeyCode::Up) {
            self.offset.1 += 1.0;
        }
        if is_key_down(KeyCode::Down) {
            self.offset.1 -= 1.0;
        }
        if is_key_down(KeyCode::Left) {
            self.offset.0 += 1.0;
        }
        if is_key_down(KeyCode::Right) {
            self.offset.0 -= 1.0;
        }
    }
    fn mouse_pan(&mut self){
        if !is_mouse_button_down(MouseButton::Left) {
            self.mouse_position = mouse_position();
        } else {
            let current_mouse = mouse_position();
            self.offset.0 += current_mouse.0 - self.mouse_position.0;
            self.offset.1 += current_mouse.1 - self.mouse_position.1;
            self.mouse_position = current_mouse;
        }
    }
    fn scroll_zoom(&mut self) {
        let scroll = mouse_wheel().1;
        if scroll != 0.0 {
        let mouse = mouse_position();
        let prev_zoom = self.zoom;
        self.zoom = (self.zoom + scroll / 10.0).clamp(0.1, 10.0);

        // Calculate the world position under the mouse before zoom
        let world_x = (mouse.0 - self.offset.0) / prev_zoom;
        let world_y = (mouse.1 - self.offset.1) / prev_zoom;

        // Adjust offset so the world position under the mouse stays under the mouse after zoom
        self.offset.0 = mouse.0 - world_x * self.zoom;
        self.offset.1 = mouse.1 - world_y * self.zoom;
        }
    }
}

fn draw_connection(cam: &Camera, gtree: &GTree, node_idx: usize) {
    let node = &gtree.nodes[node_idx];
    if let Some(parent_idx) = node.parent {
        if parent_idx >= 0 {
            let parent_idx = parent_idx as usize;
            if let Some(parent) = gtree.nodes.get(parent_idx) {
                let pos_a = vec2(node.x, node.y) * cam.zoom + vec2(cam.offset.0, cam.offset.1);
                let pos_b = vec2(parent.x, parent.y) * cam.zoom + vec2(cam.offset.0, cam.offset.1);
                let color = if parent.is_lit { YELLOW } else { DARKGRAY };
                draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 4.0, color);
            }
        }
    }
}

pub fn draw_tree(cam: &Camera, gtree: &GTree, selected_node: &mut Option<usize>) {
    // First, draw all connections
    for i in 0..gtree.nodes.len() {
        draw_connection(cam, gtree, i);
    }
    // Then, draw all nodes
    let mouse: Vec2 = mouse_position().into();
    let mut hovered_node: Option<usize> = None;

    for (i, node) in gtree.nodes.iter().enumerate() {
        if is_mouse_over_node(cam, node, mouse) {
            hovered_node = Some(i);
            if is_mouse_button_pressed(MouseButton::Left) {
                *selected_node = Some(i);
            }
        }
        draw_node(cam, node);
    }

    if let Some(i) = hovered_node {
        draw_node_hover_menu(cam, &gtree.nodes[i]);
    }
}

lazy_static::lazy_static! {
    static ref NODE_TEXTURE: Texture2D = Texture2D::from_file_with_format(include_bytes!("../assets/circle.png"), None);
}

pub fn draw_node(cam: &Camera, node: &GNode){
    let r = node.r * cam.zoom;
    let new_pos: Vec2 = vec2(node.x, node.y) * cam.zoom + vec2(cam.offset.0, cam.offset.1);
    draw_texture_ex(
        &NODE_TEXTURE,
        new_pos.x - r,
        new_pos.y - r,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(r * 2.0, r * 2.0)),
            ..Default::default()
        },
    );
}

pub fn is_mouse_over_node(cam: &Camera, node: &GNode, mouse: Vec2) -> bool {
    let node_pos = vec2(node.x, node.y) * cam.zoom + vec2(cam.offset.0, cam.offset.1);
    let r = node.r * cam.zoom;
    node_pos.distance(mouse) <= r
}

pub fn draw_node_hover_menu(cam: &Camera, node: &GNode) {
    let node_pos = vec2(node.x, node.y) * cam.zoom + vec2(cam.offset.0, cam.offset.1);
    let menu_width = 260.0;
    let padding = 12.0;
    let title_height = 28.0;
    let desc_font_size = 18.0;

    // Word wrap the description
    let wrapped = wrap_text(&node.description, menu_width - 2.0 * padding, desc_font_size);

    // Calculate height based on lines
    let desc_lines = wrapped.len() as f32;
    let menu_height = 40.0 + desc_lines * desc_font_size + padding * 2.0;

    let x = node_pos.x + 20.0;
    let y = node_pos.y - menu_height / 2.0;

    draw_rectangle(x, y, menu_width, menu_height, Color::new(0.18, 0.18, 0.22, 0.92));
    draw_text(&node.title, x + padding, y + padding + title_height, title_height, YELLOW);

    let mut y_offset = y + padding + title_height + 20.0;
    for line in wrapped {
        draw_text(&line, x + padding, y_offset, desc_font_size, LIGHTGRAY);
        y_offset += desc_font_size;
    }
}

/// Draws a persistent left-side menu with all details of a GNode.
/// Returns true if the close button was clicked.
pub fn draw_gnode_detail_menu(node: &GNode) -> bool {
    let menu_width = 400.0;
    let menu_height = screen_height() - 80.0;
    let x = 30.0;
    let y = 40.0;
    let padding = 20.0;
    let mut close_clicked = false;

    // Background
    draw_rectangle(x, y, menu_width, menu_height, Color::new(0.13, 0.13, 0.18, 0.97));

    // Close button (top right of menu)
    let close_size = 32.0;
    let close_x = x + menu_width - close_size - 10.0;
    let close_y = y + 10.0;
    draw_rectangle(close_x, close_y, close_size, close_size, RED);
    draw_text("X", close_x + 8.0, close_y + 26.0, 28.0, WHITE);

    // Detect close button click
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if mx >= close_x && mx <= close_x + close_size && my >= close_y && my <= close_y + close_size {
            close_clicked = true;
        }
    }

    // Title
    draw_text(&node.title, x + padding, y + 48.0, 32.0, YELLOW);

    // Description (wrapped)
    let desc_font_size = 20.0;
    let desc_y = y + 90.0;
    let desc_lines = wrap_text(&node.description, menu_width - 2.0 * padding, desc_font_size);
    let mut y_offset = desc_y;
    for line in desc_lines {
        draw_text(&line, x + padding, y_offset, desc_font_size, LIGHTGRAY);
        y_offset += desc_font_size + 2.0;
    }

    // Progress
    let progress_y = y_offset + 16.0;
    draw_text(
        &format!("Progress: {:.0}%", node.progress * 100.0),
        x + padding,
        progress_y,
        22.0,
        GREEN,
    );

    // Tasks
    let tasks_y = progress_y + 36.0;
    draw_text("Tasks:", x + padding, tasks_y, 22.0, SKYBLUE);
    let mut task_y = tasks_y + 28.0;
    if let Some(tasks) = &node.tasks {
        for task in tasks {
            let check = if task.checked { "[x]" } else { "[ ]" };
            draw_text(&format!("{} {}", check, task.content), x + padding + 16.0, task_y, 20.0, WHITE);
            task_y += 24.0;
        }
    } else {
        draw_text("No tasks.", x + padding + 16.0, task_y, 20.0, GRAY);
        task_y += 24.0;
    }

    // Lit status
    let lit_y = task_y + 16.0;
    let lit_text = if node.is_lit { "Lit: Yes" } else { "Lit: No" };
    let lit_color = if node.is_lit { YELLOW } else { DARKGRAY };
    draw_text(lit_text, x + padding, lit_y, 22.0, lit_color);

    close_clicked
}

/// Simple word wrap for a string, returns lines that fit within max_width for the given font size.
fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current = String::new();

    for word in words {
        let test = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        let width = measure_text(&test, None, font_size as u16, 1.0).width;
        if width > max_width && !current.is_empty() {
            lines.push(current.clone());
            current = word.to_string();
        } else {
            current = test;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
