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

#[derive(Deserialize, Serialize, Debug, DartSignal)]
pub struct ConnectOpenPiScope{
    pub host:String,
}

impl xactor::Message for ConnectOpenPiScope {
    type Result = ();
}

#[derive(Deserialize, Serialize, Debug, DartSignal)]
pub struct DisconnectOpenPiScope{
}

impl xactor::Message for DisconnectOpenPiScope {
    type Result = ();
}