use async_trait::async_trait;
use tokio::task::spawn_blocking;

use crate::api::{AsyncLooker, AsyncMover, AsyncSensor, Looker, Mover, MoveType, Sensor};
use std::sync::{Arc, Mutex};

impl<T> From<T> for AsyncRover<T>
where
    T: Sized + Mover + Looker + Sensor,
{
    fn from(sync: T) -> Self {
        AsyncRover(Arc::new(Mutex::new(sync)))
    }
}

pub struct AsyncRover<T>(Arc<Mutex<T>>);

impl<T> Clone for AsyncRover<T> {
    fn clone(&self) -> Self {
        AsyncRover(Arc::clone(&self.0))
    }
}

#[async_trait]
impl<T: 'static> AsyncMover for AsyncRover<T>
where
    T: Mover + Send,
{
    type Error = T::Error;

    async fn stop(&mut self) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.0);

        spawn_blocking(move || mover_ref.lock().unwrap().stop())
            .await
            .expect("Async wrapper error")
    }

    async fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.0);

        spawn_blocking(move || mover_ref.lock().unwrap().move_forward(speed))
            .await
            .expect("Async wrapper error")
    }

    async fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.0);

        spawn_blocking(move || mover_ref.lock().unwrap().move_backward(speed))
            .await
            .expect("Async wrapper error")
    }

    async fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.0);

        spawn_blocking(move || mover_ref.lock().unwrap().spin_right(speed))
            .await
            .expect("Async wrapper error")
    }

    async fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.0);

        spawn_blocking(move || mover_ref.lock().unwrap().spin_left(speed))
            .await
            .expect("Async wrapper error")
    }

    async fn get_move_type(&self) -> Result<MoveType, Self::Error> {
        let mover_ref = Arc::clone(&self.0);

        spawn_blocking(move || mover_ref.lock().unwrap().get_move_type())
            .await
            .expect("Async wrapper error")
    }
}

#[async_trait]
impl<T: 'static> AsyncLooker for AsyncRover<T>
where
    T: Looker + Send,
{
    type Error = T::Error;

    async fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error> {
        let looker_ref = Arc::clone(&self.0);

        spawn_blocking(move || looker_ref.lock().unwrap().look_at(h, v))
            .await
            .expect("Async wrapper error")
    }

    async fn get_look_direction(&self) -> Result<(i16, i16), Self::Error> {
        let looker_ref = Arc::clone(&self.0);

        spawn_blocking(move || looker_ref.lock().unwrap().get_look_direction())
            .await
            .expect("Async wrapper error")
    }
}

#[async_trait]
impl<T: 'static> AsyncSensor for AsyncRover<T>
where
    T: Sensor + Send,
{
    type Error = T::Error;

    async fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error> {
        let sensor_ref = Arc::clone(&self.0);

        spawn_blocking(move || sensor_ref.lock().unwrap().get_obstacles())
            .await
            .expect("Async wrapper error")
    }

    async fn get_lines(&self) -> Result<Vec<bool>, Self::Error> {
        let sensor_ref = Arc::clone(&self.0);

        spawn_blocking(move || sensor_ref.lock().unwrap().get_lines())
            .await
            .expect("Async wrapper error")
    }

    async fn scan_distance(&mut self) -> Result<f32, Self::Error> {
        let sensor_ref = Arc::clone(&self.0);

        spawn_blocking(move || sensor_ref.lock().unwrap().scan_distance())
            .await
            .expect("Async wrapper error")
    }
}
