use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug,Clone, SignalPiece)]
pub struct PiScopeServer{
   pub host:String,
   pub last_telegram:i64,
}

#[derive(Deserialize, Serialize, Debug, RustSignal)]
pub struct PiScopeServerList{
    pub servers:Vec<PiScopeServer>
}

#[derive(Deserialize, Serialize, Debug, RustSignal)]
pub struct PiScopeConnected{
    pub host:Option<String>
}