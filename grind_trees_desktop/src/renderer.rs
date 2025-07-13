use crate::app::AppState;
use macroquad::prelude::*;
use core::skill_tree::{GTree, GNode};

fn update_camera(state: &AppState) -> Camera2D {
    Camera2D {
        zoom: vec2(
            2. / screen_width() * state.zoom,
            -2. / screen_height() * state.zoom,
        ),
        target: state.pan,
        ..Default::default()
    }
}

pub fn draw(state: &mut AppState) {
    // world level drawing
    set_camera(&update_camera(&state));
    draw_circle(0.0, 0.0, 20.0, SKYBLUE);
    // ui level drawing
    set_default_camera();
    draw_text(
        &format!(
            "Mouse at: ({:.0}, {:.0})",
            state.mouse_pos.0, state.mouse_pos.1
        ),
        20.0,
        40.0,
        30.0,
        DARKGRAY,
    );
    if state.menu_on {
        // draw menu items (load, save, quit) and blur background
        draw_menu_overlay(state);
    }
    if let Some(tree) = &state.skill_tree {
        draw_skill_tree(tree);
    }
}
fn is_in_rect(mouse: Vec2, x: f32, y: f32, w: f32, h: f32) -> bool {
    mouse.x >= x && mouse.x <= x + w && mouse.y >= y && mouse.y <= y + h
}

fn draw_menu_overlay(state: &mut AppState) {
    // Dim background
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.5),
    );

    let menu_width = 300.0;
    let menu_height = 200.0;
    let x = (screen_width() - menu_width) / 2.0;
    let y = (screen_height() - menu_height) / 2.0;

    draw_rectangle(x, y, menu_width, menu_height, Color::new(0.5, 0.5, 0.5, 0.9));

    let items = [("Load", 50.0), ("Save", 100.0), ("Quit", 150.0)];

    let mouse: Vec2 = mouse_position().into();

    for (label, offset_y) in items {
        let tx = x + 20.0;
        let ty = y + offset_y;
        let tw = 100.0;
        let th = 35.0;
        if is_in_rect(mouse, tx, ty - 25.0, tw, th) {
            draw_rectangle(tx - 5.0, ty - 30.0, tw + 10.0, th, DARKGRAY);
        }
        draw_text(label, tx, ty, 30.0, WHITE);
    }

    if state.menu_on && is_mouse_button_pressed(MouseButton::Left) {
        if is_in_rect(mouse, x + 20.0, y + 25.0, 100.0, 40.0) {
            state.load = true;
        } else if is_in_rect(mouse, x + 20.0, y + 75.0, 100.0, 40.0) {
            state.save = true;
        } else if is_in_rect(mouse, x + 20.0, y + 125.0, 100.0, 40.0) {
            state.quit = true;
        }
    }
}
pub fn draw_skill_tree(tree: &GTree) {
    let screen_w = screen_width();
    let mut y = 40.0;

    // Draw tree title
    draw_text(&tree.title, screen_w * 0.5 - measure_text(&tree.title, None, 40, 1.0).width / 2.0, y, 40.0, WHITE);
    y += 60.0;

    for node in &tree.nodes {
        draw_node(node, y);
        y += 180.0; // space between nodes
    }
}

fn draw_node(node: &GNode, y: f32) {
    let x = 80.0;
    let width = screen_width() - 160.0;
    let height = 150.0;

    // Node background
    draw_rectangle(x, y, width, height, if node.is_lit { DARKGRAY } else { GRAY });

    // Node title
    draw_text(&node.title, x + 10.0, y + 30.0, 28.0, WHITE);

    // Description
    draw_text(&node.description, x + 10.0, y + 60.0, 20.0, LIGHTGRAY);

    // Progress bar background
    draw_rectangle(x + 10.0, y + 80.0, width - 20.0, 10.0, BLACK);
    // Progress bar fill
    draw_rectangle(x + 10.0, y + 80.0, (width - 20.0) * (node.progress / 100.0), 10.0, GREEN);

    // Tasks
    let mut task_y = y + 100.0;
    for task in &node.tasks {
        let checkbox = if task.checked { "[x]" } else { "[ ]" };
        let task_text = format!("{} {}", checkbox, task.content);
        draw_text(&task_text, x + 10.0, task_y, 18.0, WHITE);
        task_y += 20.0;
    }
}