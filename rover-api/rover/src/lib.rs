pub mod api {
    pub trait Rover {
        fn stop(&self);
        fn move_forward(&self, speed: u32);
        fn move_backward(&self, speed: u32);
        fn rotate_right(&self, speed: u32);
        fn rotate_left(&self, speed: u32);
    }
}

pub mod util {
    // RaspberryPi model B+ pgysical pins to BCM map
    const PIN_TO_GPIO_REV3: [i8; 41] = [
        -1, -1, -1, 2, -1, 3, -1, 4, 14, -1, 15, 17, 18, 27, -1, 22, 23, -1, 24, 10, -1, 9, 24, 11,
        7, -1, 7, -1, -1, 5, -1, 6, 12, 13, -1, 19, 16, 26, 20, -1, 21,
    ];

    // RaspberryPi model B+ BCM to physical pins map
    const GPIO_TO_PIN_REV3: [i8; 33] = [
        -1, -1, 3, 5, 7, 29, 31, 26, 24, 21, 19, 23, 32, 33, 8, 10, 36, 11, 12, 35, 38, 40, 15, 16,
        18, 22, 37, 13, -1, -1, -1, -1, 0,
    ];

    pub fn bcm2pin(gpio_id: u8) -> i8 {
        GPIO_TO_PIN_REV3[gpio_id as usize]
    }

    pub fn pin2bcm(pin_id: u8) -> i8 {
        GPIO_TO_PIN_REV3[pin_id as usize]
    }
}
