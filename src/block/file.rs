use serde::{Serialize, Deserialize};

// File to add to the block
#[derive(Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub size: [f32;2],
}
