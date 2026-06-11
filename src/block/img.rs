use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Img {
    pub path: String,
    pub size_mb: f64,
    pub size: [f32;2],
}