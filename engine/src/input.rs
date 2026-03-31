use glam::Vec2;
use std::collections::HashSet;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[allow(dead_code)]
pub struct InputState {
    keys_pressed: HashSet<KeyCode>,
    keys_held: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    mouse_pos: Vec2,
    mouse_delta: Vec2,
    mouse_buttons: [bool; 5],
    mouse_scroll: f32,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_held: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_pos: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            mouse_buttons: [false; 5],
            mouse_scroll: 0.0,
        }
    }

    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_held(&self, key: KeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn begin_frame(&mut self) {
        for key in self.keys_pressed.drain() {
            self.keys_held.insert(key);
        }
        self.keys_released.clear();
        self.mouse_delta = Vec2::ZERO;
        self.mouse_scroll = 0.0;
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event: k_event, .. } => {
                if let PhysicalKey::Code(key) = k_event.physical_key {
                    match k_event.state {
                        ElementState::Pressed => {
                            if !self.keys_held.contains(&key) {
                                self.keys_pressed.insert(key);
                                self.keys_held.insert(key);
                            }
                        }
                        ElementState::Released => {
                            self.keys_held.remove(&key);
                            self.keys_released.insert(key);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = Vec2::new(position.x as f32, position.y as f32);
                if self.mouse_pos != Vec2::ZERO {
                    self.mouse_delta = new_pos - self.mouse_pos;
                }
                self.mouse_pos = new_pos;
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, y) => self.mouse_scroll = *y,
                MouseScrollDelta::PixelDelta(pos) => self.mouse_scroll = pos.y as f32,
            },
            _ => {}
        }
    }
}
