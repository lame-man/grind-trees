mod transform;
use transform::{Camera, GNode, Task, draw_node};
use macroquad::prelude::*;

#[macroquad::main("Grind Trees")]
async fn main() {
    let mut cam: Camera = Camera::new();
    let mut tasks: Vec<Task> = Vec::new();
    let mut goals: Vec<String> = Vec::new();
    let task1: Task = Task{
        content: String::from("Type at 35 wpm"),
        checked: false
    };
    let task2: Task = Task{
        content: String::from("Solve a leetcode easy"),
        checked: false
    };
    tasks.push(task1);
    tasks.push(task2);
    goals.push(String::from("Typing Faster"));
    goals.push(String::from("Improving problem solving skills"));
    let root: GNode = GNode{
        title: String::from("Programming Novice"),
        tasks: tasks,
        goals: goals,
        x: 150.0,
        y: 150.0,
        r: 50.0,
    };
    let mut nodes: Vec<GNode> = vec![root.clone()];
    loop {
        clear_background(BLACK);
        draw_text("Grind Trees", 20.0, 40.0, 30.0, DARKGRAY);
        cam.update();
        if is_key_pressed(KeyCode::Space){
            let mouse: Vec2 = mouse_position().into();
            let mut new_node = root.clone();
            let new_pos:Vec2 = (mouse - cam.offset) / cam.zoom;
            new_node.x = new_pos.x;
            new_node.y = new_pos.y;
            nodes.push(new_node);
        }
        for node in &nodes{
            draw_node(&cam, node).await;
        }
        next_frame().await
    }
}
