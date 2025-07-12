mod gtree;
mod camera;
mod side_menu;

use gtree::{load_gtree_from_file, handle_save_shortcuts};
use camera::{Camera, draw_tree, draw_gnode_detail_menu};
use macroquad::prelude::*;

#[macroquad::main("Grind Trees")]
async fn main() {
    let mut gtree = load_gtree_from_file("new_state.json").expect("Failed to load");
    let mut cam = Camera::new();
    let mut show_side_menu = false;
    let mut selected_node: Option<usize> = None;
    let mut current_file = String::from("new_state.json");

    loop {
        clear_background(BLACK);
        draw_text("Grind Trees", 20.0, 40.0, 30.0, DARKGRAY);
        cam.update();

        draw_tree(&cam, &gtree, &mut selected_node);
        show_side_menu = side_menu::handle_side_menu(show_side_menu, gtree.progress);
        if let Some(i) = selected_node {
            let close = draw_gnode_detail_menu(&gtree.nodes[i]);
            if close {
                selected_node = None;
            }
        }
        handle_save_shortcuts(&gtree, &mut current_file);
        next_frame().await;
    }
}
