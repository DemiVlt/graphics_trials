use glutin::{
    config::ConfigTemplateBuilder,
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use std::error::Error;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

struct App {
    template: ConfigTemplateBuilder,
    window: Option<Window>,
    renderer: Option<Renderer>,
    gl_context: Option<PossiblyCurrentContext>,
    display_builder: Option<DisplayBuilder>,
    exit_state: Result<(), Box<dyn Error>>,
}

struct Renderer {
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    wbo: gl::types::GLuint,
    gl_surface: Surface<WindowSurface>,
}

impl App {
    fn new(template: ConfigTemplateBuilder, display_builder: DisplayBuilder) -> Self {
        Self {
            template,
            renderer: None,
            window: None,
            gl_context: None,
            display_builder: Some(display_builder),
            exit_state: Ok(()),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let window_attributes = Window::default_attributes()
        .with_transparent(cfg!(all(target_os = "macos", not(target_family = "wasm"))))
        .with_title("title");

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    let mut app = App::new(template, display_builder);
    event_loop.run_app(&mut app).unwrap();
}

// We're taking mostly from https://github.com/rust-windowing/glutin/blob/master/glutin_examples/src/lib.rs
// Look at there resumed function, move all the initializing you can to your resumed func, then
// that's the basics.
//
// Turns out all that sucks. Revisit OpenGL + winit once you learn opengl via glfw first
