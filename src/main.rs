/*
- Task:
    - content:string
    - checked:bool
- Gnod: (on hover show title. on click show box with tasks and goals.)
    - tasks: list[Task]
    - goals: list[string]
    - title: string
- GTree:
    - title: string
    - nodes: list[Gnod]
    - adjacency matrix: list[list[bool]]
    - progress: float (value between 0 and 1)
    - mode: int (value between 0 and 2)
- Modes:
    - edit mode: allows to create new nodes, connection, and changing the attributes of a node.
    - grind mode: locks editing. shows status bar of how far towards your goal you are(completing all nodes), which can be further split into subgoals(completing a specific branch of the tree).
    - save mode: saves the current state of the tree locally to load it later. state can be encoded as the GTree object. Current progress should be dynamically computed. and so on.
*/
use macroquad::prelude::*;

struct Task{
    content: String,
    checked: bool
}
struct GNode{
    title: String,
    tasks: Vec<Task>,
    goals: Vec<String>
}
struct GTree{
    title: String,
    nodes: Vec<GNode>,
    adjacency: Vec<Vec<bool>>,
    progress: f32
}

fn main(){
    println!("Hello World!")
}
