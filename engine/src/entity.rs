use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: u32,
    pub last_position: Vec2,
    pub position: Vec2,
    pub velocity: Vec2,
    pub last_rotation: f32,
    pub rotation: f32,
    pub scale: Vec2,
    pub color: [f32; 4],
}

pub struct EntityStore {
    entities: Vec<Entity>,
    next_id: u32,
}

impl Default for EntityStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, position: Vec2) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.push(Entity {
            id,
            last_position: position,
            position,
            velocity: Vec2::ZERO,
            last_rotation: 0.0,
            rotation: 0.0,
            scale: Vec2::ONE,
            color: [1.0, 1.0, 1.0, 1.0],
        });
        id
    }

    pub fn get(&self, id: u32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn remove(&mut self, id: u32) -> bool {
        if let Some(pos) = self.entities.iter().position(|e| e.id == id) {
            self.entities.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.iter_mut()
    }
}
