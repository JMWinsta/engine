use engine::entity::EntityStore;
use engine::input::InputState;
use engine::r#loop::run;
use engine::renderer::PrimitiveRenderer;
use glam::Vec2;
use std::time::Instant;
use winit::keyboard::KeyCode;

fn main() -> anyhow::Result<()> {
    let store_raw = std::rc::Rc::new(std::cell::RefCell::new(EntityStore::new()));

    // Spawn 5 entities at random positions
    for i in 0..5 {
        let mut store_mut = store_raw.borrow_mut();
        let id = store_mut.spawn(Vec2::new(-200.0 + (i as f32) * 100.0, 0.0));
        let e = store_mut.get_mut(id).unwrap();
        e.color = [0.2 + (i as f32) * 0.1, 0.5, 1.0, 1.0];
    }

    let mut last_print = Instant::now();
    let mut ticks = 0;

    let store_clone = store_raw.clone();
    let update = move |dt: f64, input: &InputState| {
        ticks += 1;
        if last_print.elapsed().as_secs_f32() >= 2.0 {
            println!("Physics FPS: {}", ticks as f32 / 2.0);
            ticks = 0;
            last_print = Instant::now();
        }

        let speed = 200.0;

        let mut store_mut = store_clone.borrow_mut();
        for (i, e) in store_mut.iter_mut().enumerate() {
            if i == 0 {
                // First entity controlled by WASD
                let mut move_dir = Vec2::ZERO;
                if input.is_held(KeyCode::KeyW) {
                    move_dir.y += 1.0;
                }
                if input.is_held(KeyCode::KeyS) {
                    move_dir.y -= 1.0;
                }
                if input.is_held(KeyCode::KeyA) {
                    move_dir.x -= 1.0;
                }
                if input.is_held(KeyCode::KeyD) {
                    move_dir.x += 1.0;
                }

                if move_dir != Vec2::ZERO {
                    move_dir = move_dir.normalize();
                }

                if input.is_held(KeyCode::Space) {
                    e.rotation += dt as f32 * 5.0;
                }

                e.velocity = move_dir * speed;
            } else {
                // Move in slow circle
                let time = e.position.x * 0.01 + e.position.y * 0.01;
                e.velocity = Vec2::new(time.cos(), time.sin()) * 50.0;
            }
            e.position += e.velocity * dt as f32;
        }
    };

    let store_clone2 = store_raw.clone();
    let render = move |_alpha: f64, renderer: &mut PrimitiveRenderer| {
        let store = store_clone2.borrow();
        for e in store.iter() {
            renderer.draw_rect(e.position, Vec2::new(50.0, 50.0), e.color);
            renderer.draw_line(
                e.position,
                e.position + e.velocity,
                2.0,
                [1.0, 0.0, 0.0, 1.0],
            );
        }
    };

    run(update, render)?;
    Ok(())
}
