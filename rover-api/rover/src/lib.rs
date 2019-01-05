pub mod api {
    pub trait Rover {
        fn stop(&self);
        fn move_forward(&self, speed: u32);
    }
}
