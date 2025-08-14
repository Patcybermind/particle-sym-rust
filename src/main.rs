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
    let mut particles: Vec<Particle> = (0..5_000)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() / 2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let gridx_divisions = 20;
    let gridy_divisions = 20;
    
    let wall_bounce_factor = 0.4;

    loop {
        // Initialize grid
        let mut grid: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; gridy_divisions]; gridx_divisions];
        for (i, p) in particles.iter().enumerate() {
            let gx = ((p.x / screen_width()) * gridx_divisions as f32).floor() as usize;
            let gy = ((p.y / screen_height()) * gridy_divisions as f32).floor() as usize;
            if gx < gridx_divisions && gy < gridy_divisions {
                grid[gx][gy].push(i);
            }
        }

        clear_background(BLACK);

        // Create a copy of particles for collision detection
        let particles_copy = particles.clone();
        
        for (current_index, p) in particles.iter_mut().enumerate() {
            // Update position
            p.x += p.vx;
            p.y += p.vy;

            // Apply gravity
            p.vy += 0.04;

            // Collision with other particles
            let gx = ((p.x / screen_width()) * gridx_divisions as f32).floor() as isize;
            let gy = ((p.y / screen_height()) * gridy_divisions as f32).floor() as isize;
            
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let ngx = gx + dx;
                    let ngy = gy + dy;
                    if ngx >= 0 && ngx < gridx_divisions as isize && ngy >= 0 && ngy < gridy_divisions as isize {
                        for &other_index in &grid[ngx as usize][ngy as usize] {
                            if other_index != current_index { // Avoid self-collision
                                let op = &particles_copy[other_index];
                                let dx = op.x - p.x;
                                let dy = op.y - p.y;
                                let dist_sq = dx * dx + dy * dy;
                                let min_dist = 5.0; // Minimum distance to consider a collision
                                
                                if dist_sq < min_dist * min_dist && dist_sq > 0.0 {
                                    let dist = dist_sq.sqrt();
                                    let overlap = 2.0 * (dist - min_dist);
                                    
                                    // Displace current particle
                                    p.x -= overlap * (dx / dist);
                                    p.y -= overlap * (dy / dist);
                                    
                                    // Simple elastic collision response
                                    let nx = dx / dist;
                                    let ny = dy / dist;
                                    let p1n = p.vx * nx + p.vy * ny;
                                    let p2n = op.vx * nx + op.vy * ny;
                                    let m1 = 1.0; // mass of particle 1
                                    let m2 = 1.0; // mass of particle 2
                                    let optimized_p1n = (p1n * (m1 - m2) + 2.0 * m2 * p2n) / (m1 + m2);
                                    
                                    p.vx += (optimized_p1n - p1n) * nx;
                                    p.vy += (optimized_p1n - p1n) * ny;
                                }
                            }
                        }
                    }
                }
            }

            // Collision with walls
            if p.x > screen_width() { 
                p.x = screen_width(); 
                p.vx *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }
            if p.x < 0.0 { 
                p.x = 0.0; 
                p.vx *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }
            if p.y > screen_height() { 
                p.y = screen_height(); 
                p.vy *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }
            if p.y < 0.0 { 
                p.y = 0.0; 
                p.vy *= -wall_bounce_factor * rand::gen_range(0.9, 1.1); 
            }

            draw_rectangle(p.x, p.y, 1.0, 1.0, WHITE);
        }

        next_frame().await;
    }
}