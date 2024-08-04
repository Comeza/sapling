mod user;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};
pub use user::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Paged<T> {
    offset: u32,
    data: T,
}

impl<T> Paged<T> {
    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl Paged<()> {
    pub fn new(offset: u32) -> Self {
        Self { offset, data: () }
    }
}
