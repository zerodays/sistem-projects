use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub id: i32,
    pub text: String,
}
