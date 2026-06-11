use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Text {
    pub text: String,
    pub links: Vec<(String, char, char)>,
    pub color: [u8;3],
    pub size: [f32;2],
}

impl Text {
    // create empty text
    pub fn new() -> Self {
        Text {
            text: String::new(),
            links: Vec::new(),
            color: [200,200,200],
            size: [1.0,1.0], 
        }
    }
}

