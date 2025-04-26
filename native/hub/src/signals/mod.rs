use rinf::DartSignal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, DartSignal)]
pub struct MyPreciousData {
    pub input_numbers: Vec<i32>,
    pub input_string: String,
}

impl xactor::Message for MyPreciousData {
    type Result = ();
}
