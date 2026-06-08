use serde::{Serialize, Deserialize};

// Text to add to the Block
#[derive(Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub color: [u8;3],
    pub size: [f32;2],
}

impl Text {
    // create empty text
    pub fn new() -> Self {
        Text {
            text: String::new(),
            color: [200,200,200],
            size: [1.0,1.0], 
        }
    }
}

