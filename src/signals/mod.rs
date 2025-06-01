use serde::{Deserialize, Serialize};
use xactor::message;




#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct PiScopeServer{
   pub host:String,
   pub last_telegram:i64,
}

#[message(result = "()")]
pub(crate) struct InitGui;