extern crate termion;
extern crate rover;
extern crate robohat;

use std::thread;
use std::time::Duration;
use std::io::{Write, stdout, Stdout};
use termion::{async_stdin, AsyncReader};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;

use rover::api::{Mover, Looker, Feeler};
use robohat::RobohatRover;

fn init_screen() -> (AsyncReader, RawTerminal<Stdout>) {
    let stdin = async_stdin();
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

fn drive<T: Mover + Looker + Feeler>(stdin: AsyncReader, mut stdout: RawTerminal<Stdout>, rover: &mut T) -> RawTerminal<Stdout> {
    fn print_run_params(stdout: &mut RawTerminal<Stdout>, speed: u8, pan: i16, tilt: i16) {
        write!(
            stdout,
            "{}{}Speed: {}",
            termion::cursor::Goto(1, 2),
            termion::clear::CurrentLine,
            speed
        ).unwrap();
        write!(
            stdout,
            "{}{}Looking at: [{}; {}]",
            termion::cursor::Goto(1, 3),
            termion::clear::CurrentLine,
            pan,
            tilt
        ).unwrap();
    }

    fn print_sensors(
        stdout: &mut RawTerminal<Stdout>,
        left_obstacle: bool, right_obstacle: bool,
        left_line: bool, right_line: bool,
        distance: f32,
    )
    {
        let (sx, sy) = termion::terminal_size().unwrap();

        write!(
            stdout,
            "{}Left obstacle: {}   Right obstacle: {}",
            termion::cursor::Goto(sx / 2 - 17, sy / 2),
            if left_obstacle { 1 } else { 0 }, if right_obstacle { 1 } else { 0 }
        );

        write!(
            stdout,
            "{}Left line: {}   Right line: {}",
            termion::cursor::Goto(sx / 2 - 13, sy / 2 + 1),
            if left_line { 1 } else { 0 }, if right_line { 1 } else { 0 }
        );

        write!(
            stdout,
            "{}Distance to obstacle: {:.3} m",
            termion::cursor::Goto(sx / 2 - 10, sy / 2 + 2),
            distance / 1000.0
        );
    }

    fn print_direction(
        stdout: &mut RawTerminal<Stdout>,
        dir: char,
    )
    {
        let (sx, sy) = termion::terminal_size().unwrap();

        write!(
            stdout,
            "{}",
            termion::cursor::Goto(sx / 2, sy / 2 - 1),
        ).unwrap();
        print!("{}", dir);
    }

    let mut speed: u8 = 128;
    let mut pan: i16 = 0;
    let mut tilt: i16 = 0;

    rover.look_at(pan, tilt);

    print_direction(&mut stdout, '_');
    print_run_params(&mut stdout, speed, pan, tilt);


    stdout.flush().unwrap();

    let mut keys = stdin.keys();

    loop {
        match keys.next() {
            Some(Ok(Key::Esc)) => break,
            Some(Ok(Key::PageUp)) => {
                speed = speed.saturating_add(1);
            }
            Some(Ok(Key::PageDown)) => {
                speed = speed.saturating_sub(1);
            }
            Some(Ok(Key::Left)) => {
                rover.spin_left(speed);
                print_direction(&mut stdout, '←');
            }
            Some(Ok(Key::Right)) => {
                rover.spin_right(speed);
                print_direction(&mut stdout, '→');
            }
            Some(Ok(Key::Up)) => {
                rover.move_forward(speed);
                print_direction(&mut stdout, '↑');
            }
            Some(Ok(Key::Down)) => {
                rover.move_backward(speed);
                print_direction(&mut stdout, '↓');
            }
            Some(Ok(Key::Char(' '))) => {
                rover.stop();
                print_direction(&mut stdout, '_');
            }
            Some(Ok(Key::Char('w'))) => {
                tilt = tilt.saturating_add(1);
                rover.look_at(pan, tilt);
            }
            Some(Ok(Key::Char('s'))) => {
                tilt = tilt.saturating_sub(1);
                rover.look_at(pan, tilt);
            }
            Some(Ok(Key::Char('a'))) => {
                pan = pan.saturating_add(1);
                rover.look_at(pan, tilt);
            }
            Some(Ok(Key::Char('d'))) => {
                pan = pan.saturating_sub(1);
                rover.look_at(pan, tilt);
            }
            _ => {
                let obstacles = rover.obstacles();
                let lines = rover.lines();
                print_sensors(
                    &mut stdout,
                    obstacles[0], obstacles[1],
                    lines[0], lines[1],
                    rover.get_distance()
                );
                thread::sleep(Duration::from_millis(100));
            }
        }

        print_run_params(&mut stdout, speed, pan, tilt);

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
