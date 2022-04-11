use std::time::{Duration, Instant};

use pig::Pig;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
mod pig;
mod paint_utils;



fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("piggy", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;


    // let texture_creator = canvas.texture_creator();
    // let timer = sdl_context.timer();

    let mut event_pump = sdl_context.event_pump()?;

    let mut pig = Pig::new(400,400);
    let mut last_update = Instant::now();
    let mut do_update = true;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => do_update = !do_update,

                _ => {}
            }
        }

        if last_update.elapsed() > Duration::from_millis(10) {
            if do_update {
                pig.update();
            }

            canvas.set_draw_color(Color::RGBA(0x2e, 0x34, 0x40, 255));
            canvas.clear();
            pig.draw(&mut canvas)?;
            canvas.present();

            last_update = Instant::now();
        }
    }

    Ok(())
}
