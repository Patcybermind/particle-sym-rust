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
    let mut particles: Vec<Particle> = (0..10000)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() / 2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let gridx_divisions = 15;
    let gridy_divisions = 15;
    
    let gravity_factor = 0.1; // gravity acceleration more is more gravity
    let damping_factor = 0.15; // damping factor for collisions less is more damping
    let wall_bounce_factor = 0.6;

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

            new_p.vx *= 0.99; // apply some friction
            new_p.vy *= 0.99;

            

            // gravity
            new_p.vy += gravity_factor; // apply gravity

            // collision with other particles
            // check the grid cell of the particle
            let lgx = (((new_p.x / screen_width()) * gridx_divisions as f32).floor() as isize)
                .clamp(0, gridx_divisions as isize - 1);

            let lgy = (((new_p.y / screen_height()) * gridy_divisions as f32).floor() as isize)
                .clamp(0, gridy_divisions as isize - 1);


            //for other_particle in our grid cell
            for other_particle_index in grid[lgx as usize][lgy as usize].iter() {
                if *other_particle_index == i { continue; } // skip self
                let other_p = &particles[*other_particle_index];
                
                // calculate circular distance
                let distance_to_other = ((new_p.x - other_p.x).powi(2) + (new_p.y - other_p.y).powi(2)).sqrt();
                if distance_to_other < 5.0 { // if too close, calculate the resulting velocity (depends on the starting velocity of both particles and damping amount)
                    let angle = (other_p.y - new_p.y).atan2(other_p.x - new_p.x);
                    let speed_self = (new_p.vx.powi(2) + new_p.vy.powi(2)).sqrt();
                    let speed_other = (other_p.vx.powi(2) + other_p.vy.powi(2)).sqrt();

                    // calculate new velocities based on elastic collision
                    new_p.vx += speed_other * angle.cos() * damping_factor; // damping factor
                    new_p.vy += speed_other * angle.sin() * damping_factor;
                    new_p.vx -= speed_self * angle.cos() * damping_factor; // damping factor
                    new_p.vy -= speed_self * angle.sin() * damping_factor;
                }
            }
            

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