use macroquad::prelude::*;
#[derive(Clone)]
pub struct Task{
    pub content: String,
    pub checked: bool
}
#[derive(Clone)]
pub struct GNode{
    pub title: String,
    pub tasks: Vec<Task>,
    pub goals: Vec<String>,
    pub x: f32,
    pub y: f32,
    pub r: f32,
}
pub struct GTree{
    title: String,
    nodes: Vec<GNode>,
    adjacency: Vec<Vec<bool>>,
    progress: f32
}

pub struct Camera{
    dragging:bool,
    pub zoom: f32,
    pub offset: Vec2,
    last_mouse_pos: Vec2
}
impl Camera{
    pub fn new() -> Self {
        Self {
            offset: vec2(0.0, 0.0),
            zoom: 1.0,
            dragging: false,
            last_mouse_pos: vec2(0.0, 0.0),
        }
    }
    pub fn update(&mut self) {
        let mouse = vec2(mouse_position().0, mouse_position().1);

        // --- Dragging ---
        if is_mouse_button_pressed(MouseButton::Left) {
            self.dragging = true;
            self.last_mouse_pos = mouse;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            let delta = mouse - self.last_mouse_pos;
            self.offset += delta;
            self.last_mouse_pos = mouse;
        }

        // --- Zoom ---
        let scroll = mouse_wheel().1;
        if scroll != 0.0 {
            let zoom_factor = 1.1;
            let new_zoom = if scroll > 0.0 {
                self.zoom * zoom_factor
            } else {
                self.zoom / zoom_factor
            };

            // Zoom towards mouse position
            let before = (mouse - self.offset) / self.zoom;
            self.zoom = new_zoom.clamp(0.1, 10.0);
            let after = (mouse - self.offset) / self.zoom;
            self.offset += (after - before) * self.zoom;
        }
    }
}

pub fn calc_pos(original:Vec2, zoom:f32, camera_offset:Vec2)->Vec2{
    (original * zoom) + camera_offset
}

pub fn draw_tree(cam: &Camera){
    let origin = vec2(100.0, 100.0);
    let time = get_time() as f32;
    let ball_pos = calc_pos(Vec2{x:origin.x + time.sin() * 100.0 + time*10.0,
                                                    y:origin.y + time.cos() * 100.0}, cam.zoom, cam.offset);
    let center = calc_pos(Vec2{x:origin.x + time*10.0, y:origin.y}, cam.zoom, cam.offset);
    draw_circle(ball_pos.x, ball_pos.y, 20.0 * cam.zoom, SKYBLUE);
    draw_circle(center.x, center.y, 4.0*cam.zoom, SKYBLUE);
}

pub async fn draw_node(cam: &Camera, node: &GNode){  
    let circle_texture = load_texture("assets/circle.png").await.unwrap();
    let new_pos: Vec2 = calc_pos(vec2(node.x, node.y), cam.zoom, cam.offset);
    draw_texture_ex(
        &circle_texture,
        new_pos.x,
        new_pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(node.r*cam.zoom, node.r*cam.zoom)),
            ..Default::default()
        },
    );
    //draw_circle(new_pos.x, new_pos.y,node.r*cam.zoom, YELLOW);
}
