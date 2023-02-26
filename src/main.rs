#![allow(unused)]

use std::io::stdout;
use std::{thread, time::Duration};

use crossterm::{style, QueueableCommand};

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
        stdout.queue(style::SetBackgroundColor(style::Color::Black));

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
