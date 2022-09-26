#![windows_subsystem = "windows"]

extern crate sdl2;

use std::time::Duration;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Window{
    canvas: WindowCanvas,
    //video: VideoSubsystem,
    events: EventPump
}

impl Window {
    pub fn update(&self){

    }

    pub fn render(&mut self){
        self.canvas.set_draw_color(Color::RGB(100, 149, 237));
        self.canvas.clear();
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn exit(&mut self) -> bool{
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                _ => {}
            }
        }
        false
    }
}

pub fn init() -> Window{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let win = video_subsystem.window("window", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    //window.set_bordered(false);
    let temp_canvas = win.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    Window{
        canvas: temp_canvas,
        //video: video_subsystem,
        events: event_pump
    }
}