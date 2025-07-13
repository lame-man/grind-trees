use serde::{Deserialize, Serialize};
use std::{fs, io::{self, Write}};
use macroquad::prelude::*;
use rfd::FileDialog;
/*
In this file we're going to do the data logic. including loading and saving a GTree as json.
*/
#[derive(Serialize, Deserialize)]
pub struct GTree{
    pub title: String,
    pub progress: f32,
    pub nodes: Vec<GNode>
}
#[derive(Serialize, Deserialize)]
pub struct GNode{
    pub title: String,
    pub description: String,
    pub progress: f32,
    pub tasks: Option<Vec<Task>>,
    pub is_lit: bool,
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub parent: Option<i8>,
}
#[derive(Serialize, Deserialize)]
pub struct Task{
    pub content: String,
    pub checked: bool
}
pub fn load_gtree_from_file(path: &str) -> Result<GTree, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let gtree: GTree = serde_json::from_str(&data)?;
    Ok(gtree)
}

pub fn save_gtree_to_file(gtree: &GTree, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(gtree)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn handle_save_shortcuts(gtree: &GTree, current_file: &mut String) {
    // Ctrl+S: Save
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) && !is_key_down(KeyCode::LeftShift) {
        if let Err(e) = save_gtree_to_file(gtree, current_file) {
            println!("Failed to save: {e}");
        } else {
            println!("Saved to {}", current_file);
        }
    }

    // Ctrl+Shift+S: Save As (with native file dialog)
    if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::S) {
        if let Some(path) = FileDialog::new()
            .set_title("Save As")
            .add_filter("JSON", &["json"])
            .save_file()
        {
            if let Some(path_str) = path.to_str() {
                if let Err(e) = save_gtree_to_file(gtree, path_str) {
                    println!("Failed to save: {e}");
                } else {
                    println!("Saved to {}", path_str);
                    *current_file = path_str.to_string();
                }
            }
        }
    }
}

