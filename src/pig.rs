use std::f64::consts::PI;

use rand::{thread_rng, Rng};
use sdl2::{pixels::Color, rect::Point, render::Canvas};

use crate::paint_utils::{draw_circle, TWO_PI, fill_circle};
pub struct Pig {
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32,
}

fn adjust_speed(before: i32) -> i32 {
    let out = before + thread_rng().gen_range(-3..=3);

    if out <= 0 {
        1
    } else if out >= 10 {
        10
    } else {
        out
    }
}

impl Pig {
    pub fn new(x: i32, y: i32) -> Self {
        Pig {
            x,
            y,
            speed_x: thread_rng().gen_range(1..10),
            speed_y: thread_rng().gen_range(1..10),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let x = self.x;
        let y = self.y;
        // outline
        let ear_1 = PI * 1.25;
        let ear_2 = PI * 1.75;

        // canvas.set_draw_color(Color::RGBA(0xdb, 0x70, 0x93, 0xff));
        // fill_circle(canvas, x, y, 150)?;

        canvas.set_draw_color(Color::RGBA(0xff, 0xc0, 0xcb, 255));
        {
            let radius = 150;
            assert!(radius >= 0);
            const SEGMENTS: usize = 100;
            for i in 0..SEGMENTS {
                let angle_from: f64 = i as f64 / SEGMENTS as f64 * TWO_PI;
                let angle_to: f64 = (i + 1) as f64 / SEGMENTS as f64 * TWO_PI;
                if !(ear_1 - 0.2..=ear_1 + 0.2).contains(&angle_from)
                    && !(ear_2 - 0.2..=ear_2 + 0.2).contains(&angle_from)
                    && !(ear_1 - 0.2..=ear_1 + 0.2).contains(&angle_to)
                    && !(ear_2 - 0.2..=ear_2 + 0.2).contains(&angle_to)
                {
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
            }
        };
        // nose
        draw_circle(canvas, x, y + 20, 50)?;
        draw_circle(canvas, x - 25, y, 10)?;
        draw_circle(canvas, x + 25, y, 10)?;
        // eyes
        draw_circle(canvas, x - 70, y - 40, 20)?;
        draw_circle(canvas, x + 70, y - 40, 20)?;

        // ears
        {
            let tip = Point::new(
                (self.x as f64 + (ear_1).cos() * 200.0) as i32,
                (self.y as f64 + (ear_1).sin() * 200.0) as i32,
            );
            let on_circle = Point::new(
                (self.x as f64 + (ear_1 - 0.2).cos() * 150.0) as i32,
                (self.y as f64 + (ear_1 - 0.2).sin() * 150.0) as i32,
            );
            canvas.draw_line(on_circle, tip)?;

            let on_circle = Point::new(
                (self.x as f64 + (ear_1 + 0.2).cos() * 150.0) as i32,
                (self.y as f64 + (ear_1 + 0.2).sin() * 150.0) as i32,
            );
            canvas.draw_line(on_circle, tip)?;
        }
        {
            let tip = Point::new(
                (self.x as f64 + (ear_2).cos() * 200.0) as i32,
                (self.y as f64 + (ear_2).sin() * 200.0) as i32,
            );
            let on_circle = Point::new(
                (self.x as f64 + (ear_2 - 0.2).cos() * 150.0) as i32,
                (self.y as f64 + (ear_2 - 0.2).sin() * 150.0) as i32,
            );
            canvas.draw_line(on_circle, tip)?;

            let on_circle = Point::new(
                (self.x as f64 + (ear_2 + 0.2).cos() * 150.0) as i32,
                (self.y as f64 + (ear_2 + 0.2).sin() * 150.0) as i32,
            );
            canvas.draw_line(on_circle, tip)?;
        }

        Ok(())
    }

    pub fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if self.x - 150 < 0 {
            self.speed_x = adjust_speed(self.speed_x);
        }
        if self.y - 150 < 0 {
            self.speed_y = adjust_speed(self.speed_y);
        }
        if self.x + 150 > 800 {
            self.speed_x = -adjust_speed(self.speed_x);
        }
        if self.y + 150 > 800 {
            self.speed_y = -adjust_speed(self.speed_y);
        }
    }
}
