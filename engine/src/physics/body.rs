use glam::Vec2;
use crate::physics::shapes::Shape;

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub id: usize,
    pub position: Vec2,
    pub last_position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub last_angle: f32,
    pub angular_velocity: f32,
    pub mass: f32,
    pub inv_mass: f32,
    pub inertia: f32,
    pub inv_inertia: f32,
    pub force_accumulator: Vec2,
    pub torque_accumulator: f32,
    pub restitution: f32,
    pub friction: f32,
    pub shape: Shape,
    pub is_sleeping: bool,
    pub sleep_counter: u32,
}

impl RigidBody {
    pub fn new(shape: Shape, mass: f32, position: Vec2, restitution: f32, friction: f32) -> Self {
        let inv_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        let inertia = shape.compute_inertia(mass);
        let inv_inertia = if inertia > 0.0 { 1.0 / inertia } else { 0.0 };

        Self {
            id: 0,
            position,
            last_position: position,
            velocity: Vec2::ZERO,
            angle: 0.0,
            last_angle: 0.0,
            angular_velocity: 0.0,
            mass,
            inv_mass,
            inertia,
            inv_inertia,
            force_accumulator: Vec2::ZERO,
            torque_accumulator: 0.0,
            restitution,
            friction,
            shape,
            is_sleeping: false,
            sleep_counter: 0,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if self.inv_mass > 0.0 {
            self.force_accumulator += force;
            self.is_sleeping = false;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vec2, contact_point: Vec2) {
        if self.inv_mass > 0.0 {
            self.velocity += impulse * self.inv_mass;
            self.angular_velocity += (contact_point.x * impulse.y - contact_point.y * impulse.x) * self.inv_inertia;
            self.is_sleeping = false;
        }
    }

    pub fn clear_forces(&mut self) {
        self.force_accumulator = Vec2::ZERO;
        self.torque_accumulator = 0.0;
    }
}
