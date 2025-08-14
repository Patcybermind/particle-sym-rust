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
    let mut particles: Vec<Particle> = (0..3000)
        .map(|_| Particle {
            x: rand::gen_range(0.0, screen_width()),
            y: rand::gen_range(0.0, screen_height() / 2.0),
            vx: rand::gen_range(-1.0, 1.0) * 2.0,
            vy: rand::gen_range(-1.0, 1.0) * 2.0,
        })
        .collect();

    let gridx_divisions = 15;
    let gridy_divisions = 15;
    let gravity_factor = 0.25;
    let wall_bounce_factor = 0.9;
    let min_distance = 15.0;

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
            p.vx *= 0.999; // friction
            p.vy *= 0.999;

            p.vy += gravity_factor; // gravity
            p.x += p.vx;
            p.y += p.vy;

            // wall collisions
            let offset = 2.0;
            if p.x > screen_width() - offset {
                p.x = screen_width() - offset;
                p.vx *= -wall_bounce_factor;
            }
            if p.x < offset {
                p.x = offset;
                p.vx *= -wall_bounce_factor;
            }
            if p.y > screen_height() - offset {
                p.y = screen_height() - offset;
                p.vy *= -wall_bounce_factor;
            }
            if p.y < offset {
                p.y = offset;
                p.vy *= -wall_bounce_factor;
            }
        }

        // Second pass: handle collisions
        for gx in 0..gridx_divisions {
            for gy in 0..gridy_divisions {
                let cell = &grid[gx][gy];

                // Check current cell + left/right/top/bottom neighbors
                let neighbor_offsets = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];
                for &(dx_cell, dy_cell) in &neighbor_offsets {
                    let nx = gx as isize + dx_cell;
                    let ny = gy as isize + dy_cell;

                    if nx < 0 || nx >= gridx_divisions as isize || ny < 0 || ny >= gridy_divisions as isize {
                        continue;
                    }

                    let neighbor = &grid[nx as usize][ny as usize];

                    for &a_idx in cell {
                        for &b_idx in neighbor {
                            if a_idx >= b_idx {
                                continue; // avoid double counting
                            }

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
                                let overlap = (min_distance - distance) * 0.45; 
                                let nx = dx / distance;
                                let ny = dy / distance;

                                // push apart
                                a.x -= nx * overlap;
                                a.y -= ny * overlap;
                                b.x += nx * overlap;
                                b.y += ny * overlap;

                                // velocity adjustment along collision normal
                                let dvx = b.vx - a.vx;
                                let dvy = b.vy - a.vy;
                                let rel_vel = dvx * nx + dvy * ny;

                                if rel_vel < 0.0 {
                                    let impulse = -rel_vel * 1.0; // equal mass
                                    a.vx -= impulse * nx;
                                    a.vy -= impulse * ny;
                                    b.vx += impulse * nx;
                                    b.vy += impulse * ny;
                                }
                            }
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
