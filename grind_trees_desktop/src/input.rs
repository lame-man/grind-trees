use crate::app::{self, AppState};
use macroquad::prelude::*;
use rfd::FileDialog;
use core::skill_tree::{save_tree_to_file, load_tree_from_file};

pub fn handle_input(state: &mut AppState) {
    let mouse = mouse_position();
    state.mouse_pos = mouse;

    if is_mouse_button_pressed(MouseButton::Left) {
        println!("Clicked at: {:?}", mouse);
    }
    if is_key_down(KeyCode::Up) {
        state.pan.y += state.pan_speed;
    }
    if is_key_down(KeyCode::Down) {
        state.pan.y -= state.pan_speed;
    }
    if is_key_down(KeyCode::Left) {
        state.pan.x -= state.pan_speed;
    }
    if is_key_down(KeyCode::Right) {
        state.pan.x += state.pan_speed;
    }
    if is_key_down(KeyCode::LeftShift) {
        state.zoom_speed = 2. * app::DEFAULT_ZOOM_SPEED;
        state.pan_speed = 2. * app::DEFAULT_PAN_SPEED;
    } else {
        state.zoom_speed = app::DEFAULT_ZOOM_SPEED;
        state.pan_speed = app::DEFAULT_PAN_SPEED;
    }
    if is_key_down(KeyCode::LeftControl) {
        if is_key_down(KeyCode::Equal) {
            state.zoom *= 1. + state.zoom_speed;
        }
        if is_key_down(KeyCode::Minus) {
            state.zoom *= 1. - state.zoom_speed;
        }
    }
    if is_key_pressed(KeyCode::Escape) {
        state.menu_on = !state.menu_on;
    }
    handle_menu_input(state);
}

fn handle_menu_input(state: &mut AppState) {
    if state.load {
        state.load = false;

        if let Some(path) = FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            println!("User selected file to load: {:?}", path);

            match load_tree_from_file(path.to_str().unwrap()) {
                Ok(tree) => {
                    println!("Successfully loaded skill tree: {:?}", tree.title);
                    state.skill_tree = Some(tree);
                    state.file = Some(path.to_string_lossy().into_owned());
                }
                Err(e) => {
                    println!("Failed to load skill tree: {}", e);
                }
            }
        }
    }

    if state.save {
        state.save = false;

        let path = if let Some(existing) = &state.file {
            Some(existing.clone())
        } else {
            FileDialog::new()
                .add_filter("JSON", &["json"])
                .set_file_name("export.json")
                .save_file()
                .map(|p| p.to_string_lossy().into_owned())
        };

        if let Some(path) = path {
            println!("Saving to file: {}", path);

            if let Some(tree) = &state.skill_tree {
                if let Err(e) = save_tree_to_file(tree, &path) {
                    println!("Failed to save file: {}", e);
                } else {
                    println!("Saved!");
                    state.file = Some(path);
                }
            } else {
                println!("No skill tree to save.");
            }
        }

    }

    if state.quit {
        std::process::exit(0);
    }
}
