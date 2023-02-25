#![allow(unused)]

use std::io::stdout;
use std::{thread, time::Duration};

use crossterm::{style, QueueableCommand};

fn draw_circle() {
    let diameter = 7.0;
    let radius = diameter / 2.0 - 0.5;

    let mut r: f64 = radius + 0.25;
    r = r.powf(2.0) + 1.0;

    let mut r_min: f64 = radius - 1.0;
    r_min = r_min.powf(2.0) + 1.0;

    let mut result: String = String::new();

    let mut i = 0.0;
    while i < diameter {
        let mut y: f64 = i - radius;
        y = y.powf(2.0);

        let mut j = 0.0;
        while j < diameter {
            let mut x: f64 = j - radius;
            x = x.powf(2.0);

            if r_min <= x && y <= r {
                result += "* ";
            } else {
                result += "   ";
            }

            j += 1.0;
        }

        result += "\n";
        i += 1.0;
    }

    println!("{}", result);
}

const SHAPE: &str = "
*********************
*                   *
*                   *
*                   *
*                   *
*                   *
*                   *
*                   *
*********************
";

fn main() {
    clearscreen::clear().expect("Failed to clean screen!");
    let mut stdout = stdout();

    loop {
        // PHASE 1
        // RED
        stdout.queue(style::SetBackgroundColor(style::Color::Red));
        stdout.queue(style::SetForegroundColor(style::Color::Red));

        println!("{}", SHAPE);

        // YELLOW
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Yellow));

        println!("{}", SHAPE);

        // GREEN
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Green));

        println!("{}", SHAPE);

        thread::sleep(Duration::from_secs(3));
        clearscreen::clear().expect("Failed to clean screen!");

        // PHASE 2
        // RED
        stdout.queue(style::SetBackgroundColor(style::Color::Red));
        stdout.queue(style::SetForegroundColor(style::Color::Red));

        println!("{}", SHAPE);

        // YELLOW
        stdout.queue(style::SetBackgroundColor(style::Color::Yellow));
        stdout.queue(style::SetForegroundColor(style::Color::Yellow));

        println!("{}", SHAPE);

        // GREEN
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Green));

        println!("{}", SHAPE);

        thread::sleep(Duration::from_secs_f32(1.5));
        clearscreen::clear().expect("Failed to clean screen!");

        // PHASE 3
        // RED
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Red));

        println!("{}", SHAPE);

        // YELLOW
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Yellow));

        println!("{}", SHAPE);

        // GREEN
        stdout.queue(style::SetBackgroundColor(style::Color::Green));
        stdout.queue(style::SetForegroundColor(style::Color::Green));

        println!("{}", SHAPE);

        thread::sleep(Duration::from_secs(3));
        clearscreen::clear().expect("Failed to clean screen!");

        // PHASE 4
        // RED
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Red));

        println!("{}", SHAPE);

        // YELLOW
        stdout.queue(style::SetBackgroundColor(style::Color::Yellow));
        stdout.queue(style::SetForegroundColor(style::Color::Yellow));

        println!("{}", SHAPE);

        // GREEN
        stdout.queue(style::SetBackgroundColor(style::Color::Black));
        stdout.queue(style::SetForegroundColor(style::Color::Green));

        println!("{}", SHAPE);

        thread::sleep(Duration::from_secs_f32(1.5));
        clearscreen::clear().expect("Failed to clean screen!");
    }
}
