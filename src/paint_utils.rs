use std::f64::consts::PI;

use sdl2::video::Window;
use sdl2::{rect::Point, render::Canvas};

pub const TWO_PI: f64 = 2.0 * PI;

pub fn fill_circle(canvas: &mut Canvas<Window>, x: i32, y: i32, radius: i32) -> Result<(), String> {
    assert!(radius >= 0);
    let radius_2 = (radius * radius) as f64;
    let y = y as f64;
    for dx in -radius..radius + 1 {
        let segment_length = (radius_2 - (dx * dx) as f64).sqrt();
        canvas.draw_line(
            Point::new(x + dx, (y + segment_length) as i32),
            Point::new(x + dx, (y - segment_length) as i32),
        )?;
    }
    Ok(())
}

pub fn draw_circle(canvas: &mut Canvas<Window>, x: i32, y: i32, radius: i32) -> Result<(), String> {
    assert!(radius >= 0);
    const SEGMENTS: usize = 100;
    for i in 0..SEGMENTS {
        let angle_from: f64 = i as f64 / SEGMENTS as f64 * TWO_PI;
        let angle_to: f64 = (i + 1) as f64 / SEGMENTS as f64 * TWO_PI;
        let from = Point::new(
            (x as f64 + angle_from.cos() * radius as f64) as i32,
            (y as f64 + angle_from.sin() * radius as f64) as i32,
        );

        let to = Point::new(
            (x as f64 + angle_to.cos() * radius as f64) as i32,
            (y as f64 + angle_to.sin() * radius as f64) as i32,
        );

        canvas.draw_line(from, to)?;
    }
    Ok(())
}
