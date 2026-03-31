use std::time::Instant;
use std::sync::Arc;
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
    let window = Arc::new(event_loop.create_window(window_attributes).map_err(crate::EngineError::Window)?);
    window.set_visible(true);
        
    let mut renderer = pollster::block_on(PrimitiveRenderer::new(window.clone()))?;
    let mut input = InputState::new();
    
    let mut current_time = Instant::now();
    let mut accumulator = 0.0;
    
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                renderer.resize(size.width, size.height);
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let alpha = accumulator / FIXED_TICK;
                renderer.begin_frame();
                render(alpha, &mut renderer);
                renderer.end_frame();
            }
            Event::WindowEvent { event: ref win_event, .. } => {
                input.process_event(win_event);
            }
            Event::AboutToWait => {
                let new_time = Instant::now();
                let frame_time = new_time.duration_since(current_time).as_secs_f64();
                current_time = new_time;
                
                accumulator += frame_time;
                
                while accumulator >= FIXED_TICK {
                    update(FIXED_TICK, &input);
                    input.begin_frame();
                    accumulator -= FIXED_TICK;
                }
                
                window.request_redraw();
            }
            _ => {}
        }
    }).map_err(crate::EngineError::EventLoop)?;
    
    Ok(())
}
