#![windows_subsystem = "windows"]

extern crate sdl2;

use std::{thread, time::Duration};
use sdl2::pixels::Color;

fn  main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(100, 149, 237));
    canvas.clear();
    canvas.present();
    
    thread::sleep(Duration::new(2,0));
}