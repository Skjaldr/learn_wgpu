use winit::{event::{*}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

pub struct Game {
    
}

impl Game {

    pub fn initialize(&self) {
        
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run(move |event, _, control_flow| 
            match event {
                Event::WindowEvent { 
                    window_id, 
                    event,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => ()
                }
                _ => ()

        });

    }


    pub fn run(&self) {

        // while true {
        //     self.process_input();
        //     self.update();
        //     self.render();
        // }
    }

    pub fn process_input(&self) {}
    pub fn update(&self) {}
    pub fn render(&self) {}
    pub fn destroy(&self) {}

}