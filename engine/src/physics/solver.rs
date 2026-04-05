use glam::Vec2;
use crate::physics::body::RigidBody;
use crate::physics::narrowphase::ContactManifold;

pub fn resolve_impulse(a: &mut RigidBody, b: &mut RigidBody, manifold: &ContactManifold) {
    if a.inv_mass == 0.0 && b.inv_mass == 0.0 {
        return;
    }

    let n = manifold.normal;
    let r_a = manifold.contact_point - a.position;
    let r_b = manifold.contact_point - b.position;

    let slop = 0.01;
    let percent = 0.8;
    let correction_mag = f32::max(manifold.penetration - slop, 0.0) / (a.inv_mass + b.inv_mass) * percent;
    let correction = correction_mag * n;
    
    if a.inv_mass > 0.0 {
        a.position -= correction * a.inv_mass;
    }
    if b.inv_mass > 0.0 {
        b.position += correction * b.inv_mass;
    }

    let v_a = a.velocity + Vec2::new(-a.angular_velocity * r_a.y, a.angular_velocity * r_a.x);
    let v_b = b.velocity + Vec2::new(-b.angular_velocity * r_b.y, b.angular_velocity * r_b.x);
    let relative_vel = v_b - v_a;
    let vel_along_normal = relative_vel.dot(n);

    if vel_along_normal > 0.0 {
        return;
    }

    let e = f32::min(a.restitution, b.restitution);
    let mut j = -(1.0 + e) * vel_along_normal;
    
    let cross_a = cross_2d(r_a, n);
    let cross_b = cross_2d(r_b, n);
    let inv_inertia_sum = (cross_a * cross_a * a.inv_inertia) + (cross_b * cross_b * b.inv_inertia);

    j /= a.inv_mass + b.inv_mass + inv_inertia_sum;

    let impulse = j * n;
    if a.inv_mass > 0.0 {
        a.velocity -= impulse * a.inv_mass;
        a.angular_velocity -= cross_2d(r_a, impulse) * a.inv_inertia;
        a.is_sleeping = false;
    }
    if b.inv_mass > 0.0 {
        b.velocity += impulse * b.inv_mass;
        b.angular_velocity += cross_2d(r_b, impulse) * b.inv_inertia;
        b.is_sleeping = false;
    }

    let tangent = relative_vel - relative_vel.dot(n) * n;
    if tangent.length_squared() > 0.0001 {
        let tangent = tangent.normalize();
        let mut jt = -relative_vel.dot(tangent);
        let cross_a_t = cross_2d(r_a, tangent);
        let cross_b_t = cross_2d(r_b, tangent);
        let inv_inertia_sum_t = (cross_a_t * cross_a_t * a.inv_inertia) + (cross_b_t * cross_b_t * b.inv_inertia);
        jt /= a.inv_mass + b.inv_mass + inv_inertia_sum_t;

        let mu = f32::sqrt(a.friction * b.friction);
        let friction_impulse = if jt.abs() < j * mu {
            jt * tangent
        } else {
            -j * mu * tangent
        };

        if a.inv_mass > 0.0 {
            a.velocity -= friction_impulse * a.inv_mass;
            a.angular_velocity -= cross_2d(r_a, friction_impulse) * a.inv_inertia;
        }
        if b.inv_mass > 0.0 {
            b.velocity += friction_impulse * b.inv_mass;
            b.angular_velocity += cross_2d(r_b, friction_impulse) * b.inv_inertia;
        }
    }
}

fn cross_2d(v1: Vec2, v2: Vec2) -> f32 {
    v1.x * v2.y - v1.y * v2.x
}
