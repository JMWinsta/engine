pub mod body;
pub mod shapes;
pub mod broadphase;
pub mod narrowphase;
pub mod solver;
pub mod integrator;

use glam::Vec2;
pub use body::RigidBody;
pub use shapes::{Shape, Aabb};
use broadphase::UniformGrid;
use narrowphase::detect_collision;
use solver::resolve_impulse;
use integrator::integrate;

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    pub gravity: Vec2,
    pub iterations: u32,
    pub sub_steps: u32,
    broadphase: UniformGrid,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vec2::new(0.0, -9.81),
            iterations: 8,
            sub_steps: 1,
            broadphase: UniformGrid::new(2.0),
        }
    }

    pub fn add_body(&mut self, shape: Shape, mass: f32, position: Vec2, restitution: f32, friction: f32) -> usize {
        let id = self.bodies.len();
        let mut body = RigidBody::new(shape, mass, position, restitution, friction);
        body.id = id;
        self.bodies.push(body);
        id
    }

    pub fn add_static(&mut self, shape: Shape, position: Vec2) -> usize {
        self.add_body(shape, 0.0, position, 0.5, 0.5)
    }

    pub fn step(&mut self, dt: f32) {
        let sub_dt = dt / self.sub_steps as f32;

        for _ in 0..self.sub_steps {
            for body in self.bodies.iter_mut() {
                integrate(body, sub_dt, self.gravity);
            }

            let pairs = self.broadphase.candidate_pairs(&self.bodies);
            
            let mut manifolds = Vec::new();
            for (id_a, id_b) in pairs {
                let a = &self.bodies[id_a];
                let b = &self.bodies[id_b];
                
                if let Some(manifold) = detect_collision(a, b) {
                    manifolds.push((id_a, id_b, manifold));
                }
            }

            for _ in 0..self.iterations {
                for (id_a, id_b, manifold) in manifolds.iter() {
                    let (a, b) = get_two_mut(&mut self.bodies, *id_a, *id_b);
                    resolve_impulse(a, b, manifold);
                }
            }
        }
    }
}

fn get_two_mut<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    if a < b {
        let (first, second) = slice.split_at_mut(b);
        (&mut first[a], &mut second[0])
    } else {
        let (first, second) = slice.split_at_mut(a);
        (&mut second[0], &mut first[b])
    }
}
