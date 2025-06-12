mod transform;
use transform::{Camera, GNode, GTree, load_gtree_from_file, draw_node};
use macroquad::prelude::*;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    Edit,
    Grind,
}

impl Mode {
    fn as_str(&self) -> &'static str {
        match self {
            Mode::Edit => "Edit Mode",
            Mode::Grind => "Grind Mode",
        }
    }
}

fn is_mouse_over_node(cam: &Camera, node: &GNode, mouse: Vec2) -> bool {
    let node_pos = (vec2(node.x, node.y) * cam.zoom) + cam.offset;
    let radius = node.r * cam.zoom;
    node_pos.distance(mouse) <= radius
}

fn is_mouse_over_dialogue(cam: &Camera, node: &GNode, mouse: Vec2) -> bool {
    let node_pos = (vec2(node.x, node.y) * cam.zoom) + cam.offset;
    let box_width = 260.0;
    let box_height = 120.0 + (node.tasks.len() as f32 * 20.0) + (node.goals.len() as f32 * 18.0);
    let x = node_pos.x + 10.0;
    let y = node_pos.y + 10.0;

    mouse.x >= x && mouse.x <= x + box_width && mouse.y >= y && mouse.y <= y + box_height
}

fn draw_node_dialogue(node: &GNode, position: Vec2, cam: &Camera, mode: Mode, mouse: Vec2, clicked_task: Option<usize>) {
    let box_width = 260.0;
    let box_height = 120.0 + (node.tasks.len() as f32 * 20.0) + (node.goals.len() as f32 * 18.0);
    let x = position.x + 10.0;
    let y = position.y + 10.0;

    draw_rectangle(x, y, box_width, box_height, Color::new(0.15, 0.15, 0.18, 0.95));
    draw_text(&node.title, x + 10.0, y + 28.0, 28.0, YELLOW);

    let mut y_offset = y + 50.0;
    draw_text("Tasks:", x + 10.0, y_offset, 20.0, SKYBLUE);
    y_offset += 20.0;
    for (i, task) in node.tasks.iter().enumerate() {
        let check = if task.checked { "[x]" } else { "[ ]" };
        let check_x = x + 20.0;
        let check_y = y_offset - 14.0;
        draw_text(&format!("{} {}", check, task.content), check_x, y_offset, 18.0, WHITE);

        // Draw a clickable box for the checkbox
        let box_rect = Rect::new(check_x, check_y, 22.0, 22.0);
        draw_rectangle_lines(box_rect.x, box_rect.y, box_rect.w, box_rect.h, 2.0, GRAY);

        // If this task was just clicked, draw a highlight
        if let Some(clicked) = clicked_task {
            if clicked == i {
                draw_rectangle_lines(box_rect.x-2.0, box_rect.y-2.0, box_rect.w+4.0, box_rect.h+4.0, 2.0, YELLOW);
            }
        }

        y_offset += 18.0;
    }
    y_offset += 6.0;
    draw_text("Goals:", x + 10.0, y_offset, 20.0, GREEN);
    y_offset += 20.0;
    for goal in &node.goals {
        draw_text(&format!("- {}", goal), x + 20.0, y_offset, 18.0, GRAY);
        y_offset += 18.0;
    }
}

fn save_gtree_to_file(tree: &GTree, path: &str) {
    let data = serde_json::to_string_pretty(tree).expect("Failed to serialize GTree");
    fs::write(path, data).expect("Failed to write file");
}

#[macroquad::main("Grind Trees")]
async fn main() {
    let mut cam: Camera = Camera::new();
    let mut tree = load_gtree_from_file("state.json");
    // Add mode to tree if not present (for backward compatibility)
    let mut mode = Mode::Edit;

    let mut open_dialogue: Option<usize> = None;

    loop {
        clear_background(BLACK);
        draw_text("Grind Trees", 20.0, 40.0, 30.0, DARKGRAY);
        cam.update();

        let mouse: Vec2 = mouse_position().into();

        // Toggle mode with TAB
        if is_key_pressed(KeyCode::Tab) {
            mode = if mode == Mode::Edit { Mode::Grind } else { Mode::Edit };
        }

        // Save with Shift+S
        if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::S) {
            // Save the tree with the current mode
            let mut tree_to_save = tree.clone();
            // Optionally, you could add a mode field to GTree and set it here
            save_gtree_to_file(&tree_to_save, "new_state.json");
        }

        // Draw mode on right side
        let screen_w = screen_width();
        draw_text(
            &format!("Mode: {}", mode.as_str()),
            screen_w - 225.0,
            40.0,
            32.0,
            ORANGE,
        );

        // Draw edges based on parent field
        for (i, node) in tree.nodes.iter().enumerate() {
            if let Some(parent_idx) = node.parent {
                let parent = &tree.nodes[parent_idx];
                let pos_a = (vec2(node.x, node.y) * cam.zoom) + cam.offset;
                let pos_b = (vec2(parent.x, parent.y) * cam.zoom) + cam.offset;
                draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 4.0, WHITE);
            }
        }

        // Draw all nodes
        for node in &tree.nodes {
            draw_node(&cam, node).await;
        }

        // Dialogue logic
        let mut hovered_node_idx: Option<usize> = None;

        // 1. If a dialogue is open and mouse is over it, keep it open and skip node hover checks
        if let Some(idx) = open_dialogue {
            // Only borrow mutably if we need to mutate, otherwise borrow immutably
            let mouse_over_dialogue = {
                let node = &tree.nodes[idx];
                is_mouse_over_dialogue(&cam, node, mouse)
            };
            if mouse_over_dialogue {
                // Handle checkbox clicks in grind or edit mode
                if is_mouse_button_pressed(MouseButton::Left) {
                    let node_pos = {
                        let node = &tree.nodes[idx];
                        (vec2(node.x, node.y) * cam.zoom) + cam.offset
                    };
                    let mut y_offset = node_pos.y + 10.0 + 50.0 + 20.0;
                    let mut clicked = false;
                    {
                        let node_mut = &mut tree.nodes[idx];
                        for task in node_mut.tasks.iter_mut() {
                            let check_x = node_pos.x + 10.0 + 20.0;
                            let check_y = y_offset - 14.0;
                            let box_rect = Rect::new(check_x, check_y, 22.0, 22.0);
                            if !clicked && box_rect.contains(mouse) {
                                task.checked = !task.checked;
                                clicked = true;
                                break;
                            }
                            y_offset += 18.0;
                        }
                    }
                }
                let node = &tree.nodes[idx];
                draw_node_dialogue(node, vec2(node.x, node.y), &cam, mode, mouse, None);
            } else {
                // 2. Otherwise, check if mouse is over any node
                open_dialogue = None;
                for (i, node) in tree.nodes.iter().enumerate() {
                    if is_mouse_over_node(&cam, node, mouse) {
                        hovered_node_idx = Some(i);
                    }
                }
                if let Some(idx) = hovered_node_idx {
                    open_dialogue = Some(idx);
                }
            }
        } else {
            // 3. If no dialogue is open, check for hovered node
            for (i, node) in tree.nodes.iter().enumerate() {
                if is_mouse_over_node(&cam, node, mouse) {
                    hovered_node_idx = Some(i);
                }
            }
            if let Some(idx) = hovered_node_idx {
                open_dialogue = Some(idx);
            }
        }

        // Draw dialogue if open
        if let Some(idx) = open_dialogue {
            let node = &tree.nodes[idx];
            draw_node_dialogue(node, vec2(node.x, node.y), &cam, mode, mouse, None);
        }

        next_frame().await
    }
}
