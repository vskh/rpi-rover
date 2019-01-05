pub mod api {
    pub trait Rover {
        fn move_forward(&self, speed: &u32);
    }
}
