use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub content: String,
    pub checked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GNode {
    pub title: String,
    pub description: String,
    pub progress: f32,
    pub tasks: Vec<Task>,
    pub is_lit: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GTree {
    pub title: String,
    pub progress: f32,
    pub nodes: Vec<GNode>,
}

pub fn save_tree_to_file(tree: &GTree, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = serde_json::to_string_pretty(tree)?;
    fs::write(path, json_str)?;
    Ok(())
}

pub fn load_tree_from_file(path: &str) -> Result<GTree, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(path)?;
    let tree: GTree = serde_json::from_str(&json_str)?;
    Ok(tree)
}