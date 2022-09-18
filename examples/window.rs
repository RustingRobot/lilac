#![windows_subsystem = "windows"]

extern crate sdl2;

use std::{thread, time::Duration};
use sdl2::pixels::Color;
use sdl2::event::Event;

fn  main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("window", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    //window.set_bordered(false);
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(100, 149, 237));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(100, 149, 237));
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}