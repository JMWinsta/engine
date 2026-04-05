use engine::input::InputState;
use engine::r#loop::run;
use engine::renderer::PrimitiveRenderer;
use engine::physics::{PhysicsWorld, Shape};
use glam::Vec2;
use std::rc::Rc;
use std::cell::RefCell;
use winit::keyboard::KeyCode;

fn build_scene_1(world: &mut PhysicsWorld) {
    world.bodies.clear();
    // Static floor
    world.add_static(Shape::Rect { half_extents: Vec2::new(400.0, 50.0) }, Vec2::new(0.0, -300.0));
    
    // Stack of 10 boxes
    for i in 0..10 {
        world.add_body(
            Shape::Rect { half_extents: Vec2::new(30.0, 30.0) },
            1.0,
            Vec2::new(0.0, -200.0 + i as f32 * 65.0),
            0.1,
            0.5,
        );
    }
}

fn build_scene_2(world: &mut PhysicsWorld) {
    world.bodies.clear();
    let start_pos = Vec2::new(0.0, 200.0);
    world.add_static(Shape::Circle { radius: 10.0 }, start_pos);
    
    for i in 1..=5 {
        world.add_body(
            Shape::Circle { radius: 15.0 },
            1.0,
            start_pos + Vec2::new(40.0 * i as f32, 0.0), // pull it to the side to swing
            0.5,
            0.1,
        );
    }
}

fn build_scene_3(world: &mut PhysicsWorld) {
    world.bodies.clear();
    world.add_static(Shape::Rect { half_extents: Vec2::new(400.0, 50.0) }, Vec2::new(0.0, -300.0));
    
    for i in 0..5 {
        world.add_body(
            Shape::Rect { half_extents: Vec2::new(20.0, 20.0) },
            1.0,
            Vec2::new((i as f32) * 45.0, -230.0),
            0.8,
            0.2,
        );
    }
    
    let ball_id = world.add_body(
        Shape::Circle { radius: 20.0 },
        5.0, // Heavy
        Vec2::new(-300.0, -220.0),
        0.8,
        0.2,
    );
    world.bodies[ball_id].velocity = Vec2::new(400.0, 0.0);
}

fn build_scene_4(world: &mut PhysicsWorld) {
    world.bodies.clear();
    // Scene 4: Platformer + WASD Player

    // Ground platform
    world.add_static(Shape::Rect { half_extents: Vec2::new(800.0, 30.0) }, Vec2::new(0.0, -300.0));
    
    // Platforms of different lengths and heights
    world.add_static(Shape::Rect { half_extents: Vec2::new(100.0, 15.0) }, Vec2::new(-200.0, -150.0));
    world.add_static(Shape::Rect { half_extents: Vec2::new(150.0, 15.0) }, Vec2::new(150.0, -50.0));
    world.add_static(Shape::Rect { half_extents: Vec2::new(50.0, 15.0) }, Vec2::new(350.0, 80.0));
    world.add_static(Shape::Rect { half_extents: Vec2::new(40.0, 40.0) }, Vec2::new(-100.0, 150.0));

    // Player body (Index 5 in this scene setup)
    let player_id = world.add_body(
        Shape::Rect { half_extents: Vec2::new(20.0, 35.0) },
        1.0,
        Vec2::new(0.0, -100.0),
        0.0, // inelastic so the player doesn't bounce against the floor
        0.8, // friction for walking
    );
    // Lock rotation for player
    world.bodies[player_id].inv_inertia = 0.0;

    // Add some dynamic boxes to push around
    for i in 0..4 {
        world.add_body(
            Shape::Rect { half_extents: Vec2::new(20.0, 20.0) },
            1.0,
            Vec2::new(-100.0 + i as f32 * 50.0, 250.0 + i as f32 * 50.0),
            0.2,
            0.5,
        );
    }
}


fn main() -> anyhow::Result<()> {
    let mut world = PhysicsWorld::new();
    world.gravity = Vec2::new(0.0, -800.0);
    world.iterations = 8;
    world.sub_steps = 2;
    
    // Start with the Platformer scene (Scene 4) by default
    build_scene_4(&mut world);
    
    let world_raw = Rc::new(RefCell::new(world));
    let mut current_scene = 4;

    let world_clone = world_raw.clone();
    let update = move |dt: f64, input: &InputState| {
        let mut w = world_clone.borrow_mut();
        
        let press_1 = input.is_just_pressed(KeyCode::Digit1);
        let press_2 = input.is_just_pressed(KeyCode::Digit2);
        let press_3 = input.is_just_pressed(KeyCode::Digit3);
        let press_4 = input.is_just_pressed(KeyCode::Digit4);
        
        if press_1 { build_scene_1(&mut w); current_scene = 1; }
        if press_2 { build_scene_2(&mut w); current_scene = 2; }
        if press_3 { build_scene_3(&mut w); current_scene = 3; }
        if press_4 { build_scene_4(&mut w); current_scene = 4; }

        if current_scene == 2 {
            // Spring pendulum
            let k = 10000.0;
            let rest_len = 40.0;
            for i in 0..5 {
                let id_a = i;
                let id_b = i + 1;
                let pos_a = w.bodies[id_a].position;
                let pos_b = w.bodies[id_b].position;
                let dir = pos_b - pos_a;
                let dist = dir.length();
                if dist > 0.0 {
                    let n = dir / dist;
                    let force = n * k * (dist - rest_len);
                    w.bodies[id_a].apply_force(force);
                    w.bodies[id_b].apply_force(-force);
                }
            }
        } else if current_scene == 4 {
            // Player movement logic
            let player_idx = 5;
            if player_idx < w.bodies.len() {
                let move_speed = 1000.0;
                let mut move_dir = 0.0;
                if input.is_held(KeyCode::KeyA) {
                    move_dir -= 1.0;
                }
                if input.is_held(KeyCode::KeyD) {
                    move_dir += 1.0;
                }
                w.bodies[player_idx].apply_force(Vec2::new(move_dir * move_speed, 0.0));

                let vy = w.bodies[player_idx].velocity.y;
                // Simple grounded check (velocity Y is near 0)
                if input.is_just_pressed(KeyCode::Space) && vy.abs() < 5.0 {
                    w.bodies[player_idx].apply_impulse(Vec2::new(0.0, 500.0), Vec2::ZERO);
                }
            }
        }
        
        w.step(dt as f32);
    };

    let world_clone2 = world_raw.clone();
    let render = move |alpha: f64, renderer: &mut PrimitiveRenderer| {
        let w = world_clone2.borrow();
        for body in w.bodies.iter() {
            let interp_pos = body.last_position.lerp(body.position, alpha as f32);
            let interp_angle = body.last_angle + (body.angle - body.last_angle) * (alpha as f32);
            
            match &body.shape {
                Shape::Circle { radius } => {
                    let segments = 16;
                    let mut prev_v = interp_pos + Vec2::new(radius * interp_angle.cos(), radius * interp_angle.sin());
                    for i in 1..=segments {
                        let a = interp_angle + (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
                        let next_v = interp_pos + Vec2::new(radius * a.cos(), radius * a.sin());
                        renderer.draw_line(prev_v, next_v, 2.0, [0.0, 1.0, 0.5, 1.0]);
                        prev_v = next_v;
                    }
                    if !body.is_sleeping && body.mass > 0.0 {
                        renderer.draw_line(interp_pos, prev_v, 2.0, [1.0, 0.0, 0.0, 1.0]);
                    }
                }
                Shape::Rect { half_extents } => {
                    let cos_a = interp_angle.cos();
                    let sin_a = interp_angle.sin();
                    let rotate = |v: Vec2| -> Vec2 {
                        Vec2::new(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a)
                    };
                    
                    let corners = [
                        interp_pos + rotate(Vec2::new(-half_extents.x, -half_extents.y)),
                        interp_pos + rotate(Vec2::new(half_extents.x, -half_extents.y)),
                        interp_pos + rotate(Vec2::new(half_extents.x, half_extents.y)),
                        interp_pos + rotate(Vec2::new(-half_extents.x, half_extents.y)),
                    ];
                    
                    let color = if body.inv_mass == 0.0 {
                        [0.4, 0.4, 0.4, 1.0] // Statics are gray
                    } else if body.is_sleeping {
                        [0.5, 0.5, 0.5, 1.0]
                    } else {
                        [0.8, 0.8, 0.2, 1.0]
                    };

                    for i in 0..4 {
                        renderer.draw_line(corners[i], corners[(i + 1) % 4], 2.0, color);
                    }
                }
                Shape::Polygon { vertices } => {
                    let cos_a = interp_angle.cos();
                    let sin_a = interp_angle.sin();
                    let rotate = |v: Vec2| -> Vec2 {
                        Vec2::new(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a)
                    };
                    let n = vertices.len();
                    for i in 0..n {
                        let p1 = interp_pos + rotate(vertices[i]);
                        let p2 = interp_pos + rotate(vertices[(i + 1) % n]);
                        renderer.draw_line(p1, p2, 2.0, [1.0, 0.5, 0.0, 1.0]);
                    }
                }
            }
        }
    };

    run(update, render)?;
    Ok(())
}
