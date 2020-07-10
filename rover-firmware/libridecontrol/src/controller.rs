use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use libdriver::api::{AsyncLooker, AsyncMover, AsyncSensor};

use crate::Result;

pub struct RideController<T> where T: AsyncMover + AsyncLooker + AsyncSensor {
    output: RawTerminal<Stdout>,
    rover: T,
}

impl<T> RideController<T> where T: AsyncMover + AsyncLooker + AsyncSensor {
    pub fn new(rover: T) -> Result<RideController<T>> {
        Ok(RideController {
            output: stdout().into_raw_mode()?,
            rover,
        })
    }

    fn init_screen(out: &mut dyn Write) -> Result<()> {
        write!(
            out,
            "{}{}Press 'Esc' to exit.{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )?;

        out.flush()?;

        Ok(())
    }

    fn print_run_params(out: &mut dyn Write, speed: u8, pan: i16, tilt: i16) -> Result<()> {
        write!(
            out,
            "{}{}Speed: {}",
            termion::cursor::Goto(1, 2),
            termion::clear::CurrentLine,
            speed
        )?;

        write!(
            out,
            "{}{}Looking at: [{}; {}]",
            termion::cursor::Goto(1, 3),
            termion::clear::CurrentLine,
            pan,
            tilt
        )?;

        Ok(())
    }

    fn print_sensors(
        out: &mut dyn Write,
        left_obstacle: bool,
        right_obstacle: bool,
        left_line: bool,
        right_line: bool,
        distance: f32,
    ) -> Result<()> {
        let (sx, sy) = termion::terminal_size()?;

        write!(
            out,
            "{}Left obstacle: {}   Right obstacle: {}",
            termion::cursor::Goto(sx / 2 - 17, sy / 2),
            if left_obstacle { 1 } else { 0 },
            if right_obstacle { 1 } else { 0 }
        )?;

        write!(
            out,
            "{}Left line: {}   Right line: {}",
            termion::cursor::Goto(sx / 2 - 13, sy / 2 + 1),
            if left_line { 1 } else { 0 },
            if right_line { 1 } else { 0 }
        )?;

        write!(
            out,
            "{}Distance to obstacle: {:.3} m",
            termion::cursor::Goto(sx / 2 - 14, sy / 2 + 2),
            distance / 1000.0
        )?;

        Ok(())
    }

    fn print_direction(out: &mut dyn Write, dir: char) -> Result<()> {
        let (sx, sy) = termion::terminal_size()?;

        write!(out, "{}{}", termion::cursor::Goto(sx / 2, sy / 2 - 1), dir)?;

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let out = &mut self.output;

        Self::init_screen(out)?;

        let mut speed: u8 = 128;
        let mut pan: i16 = 0;
        let mut tilt: i16 = 0;

        self.rover.look_at(pan, tilt).await?;

        Self::print_direction(out, '_')?;
        Self::print_run_params(out, speed, pan, tilt)?;

        out.flush()?;

        let mut keys = async_stdin().keys();

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
                    self.rover.spin_left(speed).await?;
                    Self::print_direction(out, '←')?;
                }
                Some(Ok(Key::Right)) => {
                    self.rover.spin_right(speed).await?;
                    Self::print_direction(out, '→')?;
                }
                Some(Ok(Key::Up)) => {
                    self.rover.move_forward(speed).await?;
                    Self::print_direction(out, '↑')?;
                }
                Some(Ok(Key::Down)) => {
                    self.rover.move_backward(speed).await?;
                    Self::print_direction(out, '↓')?;
                }
                Some(Ok(Key::Char(' '))) => {
                    self.rover.stop().await?;
                    Self::print_direction(out, '_')?;
                }
                Some(Ok(Key::Char('w'))) => {
                    tilt = tilt.saturating_add(1);
                    self.rover.look_at(pan, tilt).await?;
                }
                Some(Ok(Key::Char('s'))) => {
                    tilt = tilt.saturating_sub(1);
                    self.rover.look_at(pan, tilt).await?;
                }
                Some(Ok(Key::Char('a'))) => {
                    pan = pan.saturating_add(1);
                    self.rover.look_at(pan, tilt).await?;
                }
                Some(Ok(Key::Char('d'))) => {
                    pan = pan.saturating_sub(1);
                    self.rover.look_at(pan, tilt).await?;
                }
                _ => {
                    let obstacles = self.rover.get_obstacles().await?;
                    let lines = self.rover.get_lines().await?;
                    Self::print_sensors(
                        out,
                        obstacles[0],
                        obstacles[1],
                        lines[0],
                        lines[1],
                        self.rover.scan_distance().await?,
                    )?;
                    thread::sleep(Duration::from_millis(100));
                }
            }

            Self::print_run_params(out, speed, pan, tilt)?;

            out.flush()?;
        }

        self.rover.stop().await?;

        Ok(())
    }
}

impl<T> Drop for RideController<T> where T: AsyncMover + AsyncLooker + AsyncSensor {
    fn drop(&mut self) {
        write!(self.output, "{}", termion::cursor::Show).unwrap();
    }
}
