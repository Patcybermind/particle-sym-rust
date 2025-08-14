use macroquad::prelude::*;

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[macroquad::main("Particle System")]
async fn main() {
    let mut particles: Vec<Particle> = (0..300)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() / 2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let gridx_divisions = 15;
    let gridy_divisions = 30;
    let gravity_factor = 0.2;
    let damping_factor = 0.7;
    let wall_bounce_factor = 0.4;
    let min_distance = 8.0;

    loop {
        clear_background(BLACK);

        // Initialize grid
        let mut grid: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; gridy_divisions]; gridx_divisions];
        for (i, p) in particles.iter().enumerate() {
            let gx = ((p.x / screen_width()) * gridx_divisions as f32).floor() as usize;
            let gy = ((p.y / screen_height()) * gridy_divisions as f32).floor() as usize;
            if gx < gridx_divisions && gy < gridy_divisions {
                grid[gx][gy].push(i);
            }
        }

        // First pass: update positions and velocities
        for p in &mut particles {
            p.vx *= 0.99; // friction
            p.vy *= 0.99;

            p.vy += gravity_factor; // gravity
            p.x += p.vx;
            p.y += p.vy;

            // wall collisions
            let offset = 2.0;
            if p.x > screen_width() - offset{
                p.x = screen_width() - offset;
                p.vx *= -wall_bounce_factor * 1.0 //rand::gen_range(0.9, 1.1);
            }
            if p.x < 0.0 + offset{
                p.x = offset;
                p.vx *= -wall_bounce_factor * 1.0 //rand::gen_range(0.9, 1.1);
            }
            if p.y > screen_height() - offset{
                p.y = screen_height() - offset;
                p.vy *= -wall_bounce_factor * 1.0 //rand::gen_range(0.9, 1.1);
            }
            if p.y < 0.0 + offset {
                p.y = offset;
                p.vy *= -wall_bounce_factor * 1.0 //rand::gen_range(0.9, 1.1);
            }
        }

        // Second pass: handle collisions
        for gx in 0..gridx_divisions {
            for gy in 0..gridy_divisions {
                let cell = &grid[gx][gy];
                for i in 0..cell.len() {
                    for j in (i + 1)..cell.len() {
                        let a_idx = cell[i];
                        let b_idx = cell[j];

                        let (a, b) = if a_idx < b_idx {
                            let (left, right) = particles.split_at_mut(b_idx);
                            (&mut left[a_idx], &mut right[0])
                        } else {
                            let (left, right) = particles.split_at_mut(a_idx);
                            (&mut right[0], &mut left[b_idx])
                        };

                        let dx = b.x - a.x;
                        let dy = b.y - a.y;
                        let distance = (dx * dx + dy * dy).sqrt();

                        if distance < min_distance && distance > 0.0 {
                            let overlap = (min_distance - distance) / 2.0;
                            let nx = dx / distance;
                            let ny = dy / distance;

                            a.x -= nx * overlap;
                            a.y -= ny * overlap;
                            b.x += nx * overlap;
                            b.y += ny * overlap;

                            // Optional: dampen velocity
                            a.vx *= 0.99;
                            a.vy *= 0.99;
                            b.vx *= 0.99;
                            b.vy *= 0.99;
                        }
                    }
                }
            }
        }

        // Draw particles
        for p in &particles {
            draw_rectangle(p.x, p.y, 1.0, 1.0, WHITE);
        }

        next_frame().await;
    }
}
