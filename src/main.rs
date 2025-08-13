use macroquad::prelude::*;

// Particle struct
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[macroquad::main("Particle System")]
async fn main() {
    let mut particles: Vec<Particle> = (0..5_000)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() /2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let bounce_factor = 1.0;
    loop {
        clear_background(BLACK);

        for p in &mut particles {
            p.x += p.vx;
            p.y += p.vy;

            if p.x > screen_width() { p.x = screen_width(); p.vx *= -bounce_factor * rand::gen_range(0.9, 1.1); }
            if p.x < 0.0 { p.x = 0.0; p.vx *= -bounce_factor * rand::gen_range(0.9, 1.1); }
            if p.y > screen_height() { p.y = screen_height(); p.vy *= -bounce_factor * rand::gen_range(0.9, 1.1); }
            if p.y < 0.0 { p.y = 0.0; p.vy *= -bounce_factor * rand::gen_range(0.9, 1.1); }

            draw_circle(p.x, p.y, 1.0, WHITE);
        }

        next_frame().await;
    }
}
