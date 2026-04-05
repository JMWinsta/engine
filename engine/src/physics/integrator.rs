use glam::Vec2;
use crate::physics::body::RigidBody;

pub fn integrate(body: &mut RigidBody, dt: f32, gravity: Vec2) {
    if body.inv_mass == 0.0 {
        return;
    }
    
    if body.is_sleeping {
        return;
    }

    body.velocity += (body.force_accumulator * body.inv_mass + gravity) * dt;
    
    body.last_position = body.position;
    body.position += body.velocity * dt;

    body.angular_velocity += (body.torque_accumulator * body.inv_inertia) * dt;
    
    body.last_angle = body.angle;
    body.angle += body.angular_velocity * dt;

    body.velocity *= 0.999;
    body.angular_velocity *= 0.998;

    body.clear_forces();

    if body.velocity.length_squared() < 0.0001 && body.angular_velocity.abs() < 0.01 {
        body.sleep_counter += 1;
        if body.sleep_counter >= 60 {
            body.is_sleeping = true;
            body.velocity = Vec2::ZERO;
            body.angular_velocity = 0.0;
        }
    } else {
        body.sleep_counter = 0;
        body.is_sleeping = false;
    }
}
