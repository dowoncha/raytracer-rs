struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    frames_left: i32
}

impl Particle {
    fn animate(&mut self) {
        if self.in_use() {
            return;
        }

        --frames_left;
        x += dx;
        y += dy;
    }

    fn in_use(&self) -> bool {
        self.frames_left > 0
    }
}

const POOL_SIZE: i32 = 1000;

struct ParticlePool {
    particles: [Particle; POOL_SIZE]
}

impl ParticlePool {
    fn create(&self, x: f32, y: f32, dx: f32, dy: f32, lifetime: i32) {
        for i in 0..POOL_SIZE {
            if !particles[i].in_use() {
                particles[i]
            }
        }
    }

    fn animate() {
        for particle in particles {
            particle.animate();
        }
    }
}
