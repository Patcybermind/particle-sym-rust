use macroquad::prelude::*;

// Particle struct
#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[macroquad::main("Particle System")]
async fn main() {
    let mut particles: Vec<Particle> = (0..1)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() / 2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let gridx_divisions = 10;
    let gridy_divisions = 10;

    let wall_bounce_factor = 0.4;

    loop {
        // grid logic
        //let grid_width = screen_width() / gridx_divisions as f32; // recalculate grid sizes in case the user resized the window
        //let grid_height = screen_height() / gridy_divisions as f32;

        // initialize grid
        // x, y, particle index
        let mut grid: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; gridy_divisions]; gridx_divisions];
        for (i, p) in particles.iter().enumerate() {
            let gx = ((p.x / screen_width()) * gridx_divisions as f32).floor() as usize;
            let gy = ((p.y / screen_height()) * gridy_divisions as f32).floor() as usize;
            if gx < gridx_divisions && gy < gridy_divisions {
                grid[gx][gy].push(i);
            }
        }

        // create the new particles to write new data to
        let mut new_particles = particles.clone();

        clear_background(BLACK);

        // Process each particle: read from particles, write to new_particles
        for (i, p) in particles.iter().enumerate() {
            let new_p = &mut new_particles[i];
            
            // Update position based on current velocity
            new_p.x = p.x + p.vx;
            new_p.y = p.y + p.vy;

            // Update velocity
            new_p.vx = p.vx; // copy current velocity
            new_p.vy = p.vy; // copy current velocity

            //new_p.vx *= 0.99; // apply some friction
            //new_p.vy *= 0.99;

            // gravity
            new_p.vy += 0.04; // apply gravity

            // collision with other particles
            // check the grid cell of the particle
            let gx = ((new_p.x / screen_width()) * gridx_divisions as f32).floor() as isize;
            let gy = ((new_p.y / screen_height()) * gridy_divisions as f32).floor() as isize;

            // collision with walls
            if new_p.x > screen_width() { 
                new_p.x = screen_width(); 
                new_p.vx *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }
            if new_p.x < 0.0 { 
                new_p.x = 0.0; 
                new_p.vx *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }
            if new_p.y > screen_height() { 
                new_p.y = screen_height(); 
                new_p.vy *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }
            if new_p.y < 0.0 { 
                new_p.y = 0.0; 
                new_p.vy *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }

            // Draw using the new position
            draw_rectangle(new_p.x, new_p.y, 1.0, 1.0, WHITE);
        }

        // Replace old particles with new ones for next frame
        particles = new_particles;

        next_frame().await;
    }
}