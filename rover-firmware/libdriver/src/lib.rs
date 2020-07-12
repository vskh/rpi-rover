pub mod api;
pub mod util;

use std::error::Error;
use std::marker::{Send, Sync};

pub trait RoverError: Error + Send + Sync + 'static {}
impl<T> RoverError for T where T: Error + Send + Sync + 'static {}
