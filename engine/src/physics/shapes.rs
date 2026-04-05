use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn overlaps(&self, other: &Aabb) -> bool {
        self.max.x >= other.min.x && self.min.x <= other.max.x &&
        self.max.y >= other.min.y && self.min.y <= other.max.y
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Circle { radius: f32 },
    Rect { half_extents: Vec2 },
    Polygon { vertices: Vec<Vec2> },
}

impl Shape {
    pub fn compute_inertia(&self, mass: f32) -> f32 {
        if mass <= 0.0 {
            return 0.0;
        }
        match self {
            Shape::Circle { radius } => 0.5 * mass * radius * radius,
            Shape::Rect { half_extents } => {
                (1.0 / 12.0) * mass * (4.0 * half_extents.x * half_extents.x + 4.0 * half_extents.y * half_extents.y)
            }
            Shape::Polygon { vertices } => {
                let mut min_x = f32::MAX;
                let mut max_x = f32::MIN;
                let mut min_y = f32::MAX;
                let mut max_y = f32::MIN;
                for v in vertices {
                    min_x = min_x.min(v.x);
                    max_x = max_x.max(v.x);
                    min_y = min_y.min(v.y);
                    max_y = max_y.max(v.y);
                }
                let w = max_x - min_x;
                let h = max_y - min_y;
                (1.0 / 12.0) * mass * (w * w + h * h)
            }
        }
    }

    pub fn aabb(&self, position: Vec2, angle: f32) -> Aabb {
        match self {
            Shape::Circle { radius } => Aabb {
                min: position - Vec2::splat(*radius),
                max: position + Vec2::splat(*radius),
            },
            Shape::Rect { half_extents } => {
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                let ex = half_extents.x * cos_a.abs() + half_extents.y * sin_a.abs();
                let ey = half_extents.x * sin_a.abs() + half_extents.y * cos_a.abs();
                Aabb {
                    min: position - Vec2::new(ex, ey),
                    max: position + Vec2::new(ex, ey),
                }
            }
            Shape::Polygon { vertices } => {
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                let mut min = Vec2::splat(f32::MAX);
                let mut max = Vec2::splat(f32::MIN);
                for v in vertices {
                    let rx = v.x * cos_a - v.y * sin_a;
                    let ry = v.x * sin_a + v.y * cos_a;
                    let p = position + Vec2::new(rx, ry);
                    min = min.min(p);
                    max = max.max(p);
                }
                Aabb { min, max }
            }
        }
    }
}
