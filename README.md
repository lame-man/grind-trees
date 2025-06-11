# Grind Trees
A project about systematic grinding towards your goals.
We're using rust.
We need a way to create a window, and update the screen and allow for movement. For that we'll use macroquad. Then we want to add stuff like a GNode data type that contains tasks and achievements. The tasks should have a state of being checked (bool), and content (String).
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
# Start of the binary

- In the beginning, you always start with a root node in edit mode.
- You can then add a new node. Give it a name and so on.
