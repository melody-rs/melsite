use std::time::Instant;

pub struct Simulation {
    pub particles: Vec<Particle>,
    last_update: Instant,
}

#[derive(Default)]
pub struct Particle {
    pub position: glam::Vec2,
    pub velocity: glam::Vec2,
}

pub const GRAVITY: f32 = 9.81;
pub const PX_PER_M: f32 = 64.0;
pub const COLLISION_DAMPING: f32 = 0.5;
pub const PARTICLE_COUNT: usize = 50;
pub const PARTICLE_SIZE: f32 = 10.0;

impl Simulation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let particles_per_row = (PARTICLE_COUNT as f32).sqrt().floor();
        let particles_per_col = (PARTICLE_COUNT as f32 - 1.0) / particles_per_row + 1.0;
        let spacing = PARTICLE_SIZE * 2.0;

        let particles = (0..PARTICLE_COUNT)
            .map(|i| {
                let x = (i as f32 % particles_per_row - particles_per_row / 2.0 + 0.5) * spacing;
                let y = (i as f32 / particles_per_row - particles_per_col / 2.0 + 0.5) * spacing;
                Particle {
                    position: glam::vec2(x, y) / PX_PER_M,
                    velocity: glam::vec2(0.0, 0.0),
                }
            })
            .collect();

        Self {
            particles,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        let bounding_size = glam::vec2(640.0, 480.0) / PX_PER_M;
        let half_bounding_size = bounding_size / 2.0;

        for particle in self.particles.iter_mut() {
            particle.velocity -= glam::vec2(0.0, 1.0) * GRAVITY * delta;
            particle.position += particle.velocity * delta;

            if particle.position.x.abs() > half_bounding_size.x {
                particle.position.x = half_bounding_size.x * particle.position.x.signum();
                particle.velocity *= -1.0 * COLLISION_DAMPING;
            }
            if particle.position.y.abs() > half_bounding_size.y {
                particle.position.y = half_bounding_size.y * particle.position.y.signum();
                particle.velocity *= -1.0 * COLLISION_DAMPING;
            }
        }
    }
}
