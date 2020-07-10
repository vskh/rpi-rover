use async_trait::async_trait;
use tokio::task::spawn_blocking;

use crate::api::{AsyncMover, Mover, AsyncLooker, Looker, AsyncSensor, Sensor};
use std::sync::{Arc, Mutex};

pub struct AsyncWrap<T> {
    wrapped: Arc<Mutex<T>>
}

impl<T> AsyncWrap<T> {
    pub fn new(wrapped: T) -> AsyncWrap<T> {
        AsyncWrap {
            wrapped: Arc::new(Mutex::new(wrapped))
        }
    }
}

#[async_trait]
impl<T: 'static> AsyncMover for AsyncWrap<T> where T: Mover + Send {
    type Error = T::Error;

    async fn stop(&mut self) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { mover_ref.lock().unwrap().stop() })
            .await
            .expect("Async wrapper error")
    }

    async fn move_forward(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { mover_ref.lock().unwrap().move_forward(speed) })
            .await
            .expect("Async wrapper error")
    }

    async fn move_backward(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { mover_ref.lock().unwrap().move_backward(speed) })
            .await
            .expect("Async wrapper error")
    }

    async fn spin_right(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { mover_ref.lock().unwrap().spin_right(speed) })
            .await
            .expect("Async wrapper error")
    }

    async fn spin_left(&mut self, speed: u8) -> Result<(), Self::Error> {
        let mover_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { mover_ref.lock().unwrap().spin_left(speed) })
            .await
            .expect("Async wrapper error")
    }
}

#[async_trait]
impl<T: 'static> AsyncLooker for AsyncWrap<T> where T: Looker + Send {
    type Error = T::Error;

    async fn look_at(&mut self, h: i16, v: i16) -> Result<(), Self::Error> {
        let looker_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { looker_ref.lock().unwrap().look_at(h, v) })
            .await
            .expect("Async wrapper error")
    }
}

#[async_trait]
impl<T:'static> AsyncSensor for AsyncWrap<T> where T: Sensor + Send {
    type Error = T::Error;

    async fn get_obstacles(&self) -> Result<Vec<bool>, Self::Error> {
        let sensor_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { sensor_ref.lock().unwrap().get_obstacles() })
            .await
            .expect("Async wrapper error")
    }

    async fn get_lines(&self) -> Result<Vec<bool>, Self::Error> {
        let sensor_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { sensor_ref.lock().unwrap().get_lines() })
            .await
            .expect("Async wrapper error")
    }

    async fn scan_distance(&mut self) -> Result<f32, Self::Error> {
        let sensor_ref = Arc::clone(&self.wrapped);

        spawn_blocking(move || { sensor_ref.lock().unwrap().scan_distance() })
            .await
            .expect("Async wrapper error")
    }
}