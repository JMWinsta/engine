use std::sync::Arc;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::input::InputState;
use crate::renderer::PrimitiveRenderer;

const FIXED_TICK: f64 = 1.0 / 60.0;

pub fn run(
    mut update: impl FnMut(f64, &InputState) + 'static,
    mut render: impl FnMut(f64, &mut PrimitiveRenderer) + 'static,
) -> Result<(), crate::EngineError> {
    let event_loop = EventLoop::new().map_err(crate::EngineError::EventLoop)?;
    let window_attributes = Window::default_attributes().with_title("Engine Sandbox");
    let window = Arc::new(
        event_loop
            .create_window(window_attributes)
            .map_err(crate::EngineError::Window)?,
    );
    window.set_visible(true);

    let mut renderer = pollster::block_on(PrimitiveRenderer::new(window.clone()))?;
    let mut input = InputState::new();

    let mut current_time = Instant::now();
    let mut accumulator = 0.0;

    let mut frame_times = [0.0; 60];
    let mut frame_count: usize = 0;

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.resize(size.width, size.height);
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let alpha = accumulator / FIXED_TICK;
                renderer.begin_frame();
                render(alpha, &mut renderer);
                renderer.end_frame();
            }
            Event::WindowEvent {
                event: ref win_event,
                ..
            } => {
                input.process_event(win_event);
            }
            Event::AboutToWait => {
                let new_time = Instant::now();
                let frame_time = new_time.duration_since(current_time).as_secs_f64();
                current_time = new_time;

                frame_times[frame_count % 60] = frame_time;
                frame_count += 1;

                let num_frames_recorded = frame_count.min(60);
                let avg_frame_time = frame_times[..num_frames_recorded].iter().sum::<f64>() / num_frames_recorded as f64;
                let avg_fps = if avg_frame_time > 0.0 { 1.0 / avg_frame_time } else { 0.0 };

                accumulator += frame_time;

                let expected_ticks = (accumulator / FIXED_TICK).floor() as u32;
                if expected_ticks >= 3 {
                    println!("SPIRAL OF DEATH: frame took too long, clamping ticks");
                    accumulator = 2.0 * FIXED_TICK;
                }

                let mut ticks_this_frame = 0;
                while accumulator >= FIXED_TICK {
                    update(FIXED_TICK, &input);
                    input.begin_frame();
                    accumulator -= FIXED_TICK;
                    ticks_this_frame += 1;
                }

                let alpha = accumulator / FIXED_TICK;
                println!(
                    "Debug Overlay | Real time: {:.2}ms | Ticks: {} | Render Alpha: {:.2} | Avg FPS: {:.1}",
                    frame_time * 1000.0,
                    ticks_this_frame,
                    alpha,
                    avg_fps
                );

                window.request_redraw();
            }
            _ => {}
        })
        .map_err(crate::EngineError::EventLoop)?;

    Ok(())
}
