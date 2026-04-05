use crate::physics::body::RigidBody;
use std::collections::HashMap;

pub struct UniformGrid {
    pub cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl UniformGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn candidate_pairs(&mut self, bodies: &[RigidBody]) -> Vec<(usize, usize)> {
        self.cells.clear();

        for (id, body) in bodies.iter().enumerate() {
            let aabb = body.shape.aabb(body.position, body.angle);
            let min_x = (aabb.min.x / self.cell_size).floor() as i32;
            let min_y = (aabb.min.y / self.cell_size).floor() as i32;
            let max_x = (aabb.max.x / self.cell_size).floor() as i32;
            let max_y = (aabb.max.y / self.cell_size).floor() as i32;

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    self.cells.entry((x, y)).or_default().push(id);
                }
            }
        }

        let mut pairs = Vec::new();
        for ids in self.cells.values() {
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    let id_a = ids[i];
                    let id_b = ids[j];
                    let a = id_a.min(id_b);
                    let b = id_a.max(id_b);
                    if bodies[a].inv_mass == 0.0 && bodies[b].inv_mass == 0.0 {
                        continue;
                    }
                    let pair = (a, b);
                    if !pairs.contains(&pair) {
                        pairs.push(pair);
                    }
                }
            }
        }

        pairs.retain(|&(a, b)| {
            let aabb_a = bodies[a].shape.aabb(bodies[a].position, bodies[a].angle);
            let aabb_b = bodies[b].shape.aabb(bodies[b].position, bodies[b].angle);
            aabb_a.overlaps(&aabb_b)
        });

        pairs
    }
}
