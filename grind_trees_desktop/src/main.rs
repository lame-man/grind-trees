use macroquad::prelude::*;

mod app;
mod renderer;
mod input;

#[macroquad::main("Grind Trees")]
async fn main() {
    let mut state = app::AppState::new(); // we could set default zoom and pan if needed
    loop {
        input::handle_input(&mut state);
        clear_background(BLACK);
        renderer::draw(&mut state);
        next_frame().await;
    }
}

