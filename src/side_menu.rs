use macroquad::prelude::*;

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

pub fn draw_side_menu(progress: f32) {
    let menu_width = 300.0;
    let menu_height = 120.0;
    let margin = 40.0;
    let x = screen_width() - menu_width - margin;
    let y = margin;
    draw_rectangle(
        x,
        y,
        menu_width,
        menu_height,
        Color::new(0.12, 0.12, 0.15, 0.75), // More transparent
    );

    // Progress bar
    let progress = progress.clamp(0.0, 1.0);
    let bar_x = x + 30.0;
    let bar_y = y + 60.0;
    let bar_w = menu_width - 60.0;
    let bar_h = 28.0;
    let color = lerp_color(RED, GREEN, progress);

    draw_rectangle(bar_x, bar_y, bar_w, bar_h, DARKGRAY);
    draw_rectangle(bar_x, bar_y, bar_w * progress, bar_h, color);

    draw_text(
        &format!("Total Progress: {:.0}%", progress * 100.0),
        x + 30.0,
        y + 40.0,
        32.0,
        WHITE,
    );
}

/// Handles toggling and drawing the side menu.
/// Returns the new state of show_side_menu.
pub fn handle_side_menu(mut show_side_menu: bool, progress: f32) -> bool {
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::P) {
        show_side_menu = !show_side_menu;
    }
    if show_side_menu {
        draw_side_menu(progress);
    }
    show_side_menu
}