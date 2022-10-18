use fyrox::core::parking_lot::Mutex;
use fyrox::{
    core::pool::Handle,
    engine::{resource_manager::ResourceManager, Engine, EngineInitParams, SerializationContext},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        grid::{Column, GridBuilder, Row},
        message::{MessageDirection, UiMessage},
        ttf::{Font, SharedFont},
        widget::{WidgetBuilder, WidgetMessage},
        window::{WindowBuilder, WindowMessage, WindowTitle},
        Thickness, UiNode, UserInterface,
    },
};
use std::{
    path::Path,
    sync::{mpsc::Sender, Arc, RwLock},
};

fn main() {
    let events_loop = EventLoop::<()>::new();

    let primary_monitor = events_loop.primary_monitor().unwrap();

    let mut monitor_dimensions = primary_monitor.size();
    monitor_dimensions.height = (monitor_dimensions.height as f32 * 0.7) as u32;
    monitor_dimensions.width = (monitor_dimensions.width as f32 * 0.7) as u32;

    let inner_size = monitor_dimensions.to_logical::<f32>(primary_monitor.scale_factor());

    let window_builder = fyrox::window::WindowBuilder::new()
        .with_title("Mil Std 1553B Sim")
        .with_inner_size(inner_size)
        .with_resizable(true);

    let serialization_context = Arc::new(SerializationContext::new());

    let mut engine = Engine::new(EngineInitParams {
        window_builder,
        resource_manager: ResourceManager::new(serialization_context.clone()),
        serialization_context,
        events_loop: &events_loop,
        vsync: false,
    })
    .unwrap();

    events_loop.run(move |event, _, control_flow| {
        // game.process_input_event(&event);

        match event {
            Event::MainEventsCleared => {
                engine.get_window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                engine.render().unwrap();
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::Resized(new_size) => {
                    engine.set_frame_size(new_size.into()).unwrap();
                },
                _ => (),
            },
            Event::LoopDestroyed => {

            },
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}
