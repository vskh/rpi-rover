extern crate termion;
extern crate rover;
extern crate robohat;

use std::io::{Write, stdout, stdin, Stdin, Stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use rover::api::Rover;
use robohat::RobohatRover;

fn init_screen() -> (Stdin, RawTerminal<Stdout>) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}Press 'Esc' to exit.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide
    ).unwrap();
    stdout.flush().unwrap();

    (stdin, stdout)
}

fn drive(stdin: Stdin, mut stdout: RawTerminal<Stdout>, rover: &mut dyn Rover) -> RawTerminal<Stdout> {
    let (sx, sy) = termion::terminal_size().unwrap();
    let mut speed: f32 = 0.0;

    write!(
        stdout,
        "{}{}Speed: {}",
        termion::cursor::Goto(1, 2),
        termion::clear::CurrentLine,
        speed
    ).unwrap();
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(sx / 2, sy / 2),
        termion::clear::CurrentLine
    ).unwrap();
    println!("_");
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}", termion::cursor::Goto(sx / 2, sy / 2)).unwrap();

        match c.unwrap() {
            Key::Esc => break,
            Key::PageUp => {
                speed += 0.1;
                if speed > 1.0 {
                    speed = 1.0;
                }
            }
            Key::PageDown => {
                speed -= 0.1;
                if speed < 0.0 {
                    speed = 0.0;
                }
            }
            Key::Left => {
                rover.spin_left(speed);
                println!("←");
            }
            Key::Right => {
                rover.spin_right(speed);
                println!("→");
            }
            Key::Up => {
                rover.move_forward(speed);
                println!("↑");
            }
            Key::Down => {
                rover.move_backward(speed);
                println!("↓");
            }
            Key::Char(' ') => {
                rover.stop();
                println!("_");
            }
            _ => {}
        }

        write!(
            stdout,
            "{}{}Speed: {}",
            termion::cursor::Goto(1, 2),
            termion::clear::CurrentLine,
            speed
        ).unwrap();

        stdout.flush().unwrap();
    }

    rover.stop();

    stdout
}

fn cleanup(mut stdout: RawTerminal<Stdout>) {
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn main() {
    let mut rover = RobohatRover::new().unwrap();

    let (stdin, mut stdout) = init_screen();
    stdout = drive(stdin, stdout, &mut rover);
    cleanup(stdout);
}
