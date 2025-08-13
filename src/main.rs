use macroquad::prelude::*;
use rayon::prelude::*;
use macroquad::models::{Vertex, Mesh};

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
            y: rand::gen_range(0.0, screen_height()),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    loop {
        clear_background(BLACK);

        // Update particles in parallel
        particles.par_iter_mut().for_each(|p| {
            p.x += p.vx;
            p.y += p.vy;

            if p.x > screen_width() { p.x = screen_width(); p.vx *= -1.0; }
            if p.x < 0.0 { p.x = 0.0; p.vx *= -1.0; }
            if p.y > screen_height() { p.y = screen_height(); p.vy *= -1.0; }
            if p.y < 0.0 { p.y = 0.0; p.vy *= -1.0; }
        });

        // Create vertices for all particles
        let vertices: Vec<Vertex> = particles.iter().map(|p| Vertex {
            position: [p.x, p.y, 0.0],
            uv: [0.0, 0.0],
            color: WHITE.into(),
        }).collect();

        // Indices for points
        let indices: Vec<u16> = (0..particles.len() as u16).collect();

        // Draw all particles as points using a Mesh
        let mesh = Mesh {
            vertices,
            indices,
        };
        draw_mesh(&mesh);

        next_frame().await;
    }
}
