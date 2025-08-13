use macroquad::prelude::*;
use rayon::prelude::*;


// Particle struct
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}



#[macroquad::main("Particle System")]
async fn main() {
    let mut particles: Vec<Particle> = (0..200_000)
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

        // prcocesses in parrallel
        particles.par_iter_mut().for_each(|p| {
            p.x += p.vx;
            p.y += p.vy;

            if p.x > screen_width() { p.x = screen_width(); p.vx *= -1.0; }
            if p.x < 0.0 { p.x = 0.0; p.vx *= -1.0; }
            if p.y > screen_height() { p.y = screen_height(); p.vy *= -1.0; }
            if p.y < 0.0 { p.y = 0.0; p.vy *= -1.0; }
        });

        
        let mut vertices = Vec::with_capacity(particles.len() * 4); // 4 vertices per quad
            
        for p in &particles {
            vertices.push(Vertex { position: vec2(p.x, p.y), color: WHITE });
            vertices.push(Vertex { position: vec2(p.x+1.0, p.y), color: WHITE });
            vertices.push(Vertex { position: vec2(p.x+1.0, p.y+1.0), color: WHITE });
            vertices.push(Vertex { position: vec2(p.x, p.y+1.0), color: WHITE });
        }
        
        draw_mesh(&Mesh { vertices, indices: (0..vertices.len() as u16).collect(), mode: MeshMode::Triangles, texture: None });

        next_frame().await;
    }
}
