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
    let mut particles: Vec<Particle> = (0..100_000)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() /2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let gridx_divisions = 10;
    let gridy_divisions = 10;
    
    let wall_bounce_factor = 1.0;

    let mut grid: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; gridy_divisions]; gridx_divisions];


    loop {
        // grid logic
        //let grid_width = screen_width() / gridx_divisions as f32; // recalculate grid sizes in case the user resized the window
        //let grid_height = screen_height() / gridy_divisions as f32;

        // initialize grid
        // x, y, particle index


        clear_background(BLACK);

        for p in &mut particles {
            p.x += p.vx;
            p.y += p.vy;

            if p.x > screen_width() { p.x = screen_width(); p.vx *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); }
            if p.x < 0.0 { p.x = 0.0; p.vx *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); }
            if p.y > screen_height() { p.y = screen_height(); p.vy *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); }
            if p.y < 0.0 { p.y = 0.0; p.vy *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); }

            draw_rectangle(p.x, p.y, 1.0, 1.0, WHITE);

        }

        next_frame().await;
    }
}
