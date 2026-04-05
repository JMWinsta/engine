use glam::Vec2;
use crate::physics::body::RigidBody;
use crate::physics::shapes::Shape;

#[derive(Debug)]
pub struct ContactManifold {
    pub normal: Vec2,
    pub penetration: f32,
    pub contact_point: Vec2,
}

pub fn detect_collision(a: &RigidBody, b: &RigidBody) -> Option<ContactManifold> {
    match (&a.shape, &b.shape) {
        (Shape::Circle { radius: ra }, Shape::Circle { radius: rb }) => {
            circle_vs_circle(a.position, *ra, b.position, *rb)
        }
        (Shape::Rect { half_extents: ha }, Shape::Rect { half_extents: hb }) => {
            if a.angle == 0.0 && b.angle == 0.0 {
                aabb_vs_aabb(a.position, *ha, b.position, *hb)
            } else {
                let poly_a = rect_to_poly(a.position, *ha, a.angle);
                let poly_b = rect_to_poly(b.position, *hb, b.angle);
                polygon_vs_polygon(&poly_a, &poly_b)
            }
        }
        (Shape::Circle { radius }, Shape::Rect { half_extents }) => {
            if b.angle == 0.0 {
                circle_vs_aabb(a.position, *radius, b.position, *half_extents, false)
            } else {
                let rot_a = rotate(a.position - b.position, -b.angle);
                if let Some(mut m) = circle_vs_aabb(b.position + rot_a, *radius, b.position, *half_extents, false) {
                    m.normal = rotate(m.normal, b.angle);
                    m.contact_point = b.position + rotate(m.contact_point - b.position, b.angle);
                    Some(m)
                } else {
                    None
                }
            }
        }
        (Shape::Rect { half_extents }, Shape::Circle { radius }) => {
            if a.angle == 0.0 {
                circle_vs_aabb(b.position, *radius, a.position, *half_extents, true)
            } else {
                let rot_b = rotate(b.position - a.position, -a.angle);
                if let Some(mut m) = circle_vs_aabb(a.position + rot_b, *radius, a.position, *half_extents, true) {
                    m.normal = rotate(m.normal, a.angle);
                    m.contact_point = a.position + rotate(m.contact_point - a.position, a.angle);
                    Some(m)
                } else {
                    None
                }
            }
        }
        (Shape::Polygon { vertices: va }, Shape::Polygon { vertices: vb }) => {
            let poly_a = transform_poly(va, a.position, a.angle);
            let poly_b = transform_poly(vb, b.position, b.angle);
            polygon_vs_polygon(&poly_a, &poly_b)
        }
        (Shape::Polygon { vertices }, Shape::Rect { half_extents }) => {
            let poly_a = transform_poly(vertices, a.position, a.angle);
            let poly_b = rect_to_poly(b.position, *half_extents, b.angle);
            polygon_vs_polygon(&poly_a, &poly_b)
        }
        (Shape::Rect { half_extents }, Shape::Polygon { vertices }) => {
            let poly_a = rect_to_poly(a.position, *half_extents, a.angle);
            let poly_b = transform_poly(vertices, b.position, b.angle);
            polygon_vs_polygon(&poly_a, &poly_b)
        }
        (Shape::Circle { radius: _ }, Shape::Polygon { vertices: _ }) => {
            // Simplified: treat circle as point vs poly for now, or use SAT with circle.
            // For a complete physics engine, Circle vs Poly uses SAT with poly normals + normal to closest vertex.
            // For now, let's omit or approximate since prompt didn't explicitly ask for Circle vs Poly
            None
        }
        (Shape::Polygon { vertices: _ }, Shape::Circle { radius: _ }) => {
            None
        }
    }
}

fn rotate(v: Vec2, angle: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

fn rect_to_poly(pos: Vec2, he: Vec2, angle: f32) -> Vec<Vec2> {
    let corners = [
        Vec2::new(-he.x, -he.y),
        Vec2::new(he.x, -he.y),
        Vec2::new(he.x, he.y),
        Vec2::new(-he.x, he.y),
    ];
    corners.iter().map(|&v| pos + rotate(v, angle)).collect()
}

fn transform_poly(verts: &[Vec2], pos: Vec2, angle: f32) -> Vec<Vec2> {
    verts.iter().map(|&v| pos + rotate(v, angle)).collect()
}

fn circle_vs_circle(pos_a: Vec2, ra: f32, pos_b: Vec2, rb: f32) -> Option<ContactManifold> {
    let diff = pos_b - pos_a;
    let dist_sq = diff.length_squared();
    let radii = ra + rb;
    if dist_sq >= radii * radii {
        return None;
    }
    let dist = dist_sq.sqrt();
    let normal = if dist > 0.0 { diff / dist } else { Vec2::new(1.0, 0.0) };
    Some(ContactManifold {
        normal,
        penetration: radii - dist,
        contact_point: pos_a + normal * ra,
    })
}

fn aabb_vs_aabb(pos_a: Vec2, he_a: Vec2, pos_b: Vec2, he_b: Vec2) -> Option<ContactManifold> {
    let n = pos_b - pos_a;
    let x_overlap = he_a.x + he_b.x - n.x.abs();
    if x_overlap > 0.0 {
        let y_overlap = he_a.y + he_b.y - n.y.abs();
        if y_overlap > 0.0 {
            if x_overlap < y_overlap {
                let normal = if n.x < 0.0 { Vec2::new(-1.0, 0.0) } else { Vec2::new(1.0, 0.0) };
                let cp = if n.x < 0.0 { pos_a - Vec2::new(he_a.x, 0.0) } else { pos_a + Vec2::new(he_a.x, 0.0) };
                return Some(ContactManifold {
                    normal,
                    penetration: x_overlap,
                    contact_point: cp, // simplified
                });
            } else {
                let normal = if n.y < 0.0 { Vec2::new(0.0, -1.0) } else { Vec2::new(0.0, 1.0) };
                let cp = if n.y < 0.0 { pos_a - Vec2::new(0.0, he_a.y) } else { pos_a + Vec2::new(0.0, he_a.y) };
                return Some(ContactManifold {
                    normal,
                    penetration: y_overlap,
                    contact_point: cp, // simplified
                });
            }
        }
    }
    None
}

fn circle_vs_aabb(pos_c: Vec2, r: f32, pos_r: Vec2, he: Vec2, flipped: bool) -> Option<ContactManifold> {
    let diff = pos_c - pos_r;
    let clamped = Vec2::new(
        diff.x.clamp(-he.x, he.x),
        diff.y.clamp(-he.y, he.y)
    );
    let mut closest = pos_r + clamped;
    let mut d = pos_c - closest;
    
    // If center is inside AABB
    if d == Vec2::ZERO {
        if he.x - diff.x.abs() < he.y - diff.y.abs() {
            if diff.x > 0.0 { closest.x = pos_r.x + he.x; } else { closest.x = pos_r.x - he.x; }
        } else {
            if diff.y > 0.0 { closest.y = pos_r.y + he.y; } else { closest.y = pos_r.y - he.y; }
        }
        d = pos_c - closest;
    }

    let dist_sq = d.length_squared();
    if dist_sq <= r * r {
        let dist = dist_sq.sqrt();
        let normal = if dist > 0.0 { d / dist } else { Vec2::new(1.0, 0.0) };
        let mut n = normal;
        let p = r - dist;
        if flipped {
            n = -n;
        }
        return Some(ContactManifold {
            normal: n,
            penetration: p,
            contact_point: closest,
        });
    }
    None
}

fn polygon_vs_polygon(poly_a: &[Vec2], poly_b: &[Vec2]) -> Option<ContactManifold> {
    let mut min_pen = f32::MAX;
    let mut best_normal = Vec2::ZERO;

    for (a, b) in [ (poly_a, poly_b), (poly_b, poly_a) ].iter() {
        let n = a.len();
        for i in 0..n {
            let v1 = a[i];
            let v2 = a[(i + 1) % n];
            let edge = v2 - v1;
            let normal = Vec2::new(-edge.y, edge.x).normalize();

            let mut min_a = f32::MAX;
            let mut max_a = f32::MIN;
            for v in a.iter() {
                let p = v.dot(normal);
                min_a = min_a.min(p);
                max_a = max_a.max(p);
            }

            let mut min_b = f32::MAX;
            let mut max_b = f32::MIN;
            for v in b.iter() {
                let p = v.dot(normal);
                min_b = min_b.min(p);
                max_b = max_b.max(p);
            }

            if max_a <= min_b || max_b <= min_a {
                return None; // separating axis found
            }

            let pen = (max_a.min(max_b)) - (min_a.max(min_b));
            if pen < min_pen {
                min_pen = pen;
                best_normal = normal;
                if std::ptr::eq(*a, poly_b) {
                    best_normal = -best_normal;
                }
            }
        }
    }

    // Ensure normal points from A to B
    // Just find a simple contact point, approximate with center difference
    let center_a: Vec2 = poly_a.iter().sum::<Vec2>() / poly_a.len() as f32;
    let center_b: Vec2 = poly_b.iter().sum::<Vec2>() / poly_b.len() as f32;
    if (center_b - center_a).dot(best_normal) < 0.0 {
        best_normal = -best_normal;
    }

    // Extremely simplified contact point for Poly vs Poly
    let cp = center_a + best_normal * min_pen * 0.5;

    Some(ContactManifold {
        normal: best_normal,
        penetration: min_pen,
        contact_point: cp,
    })
}
