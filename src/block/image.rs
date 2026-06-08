use serde::{Serialize, Deserialize};

// Image to add to the Block
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub path: String,
    pub size: [f32;2],
}
