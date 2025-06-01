use anyhow::Ok;
use xactor::{message, Actor, Addr, Context, Handler};

use crate::{generated::open_pi_scope::{EulerAngle, GnssData, Quaternion}, signals::PiScopeServer};

use super::{gui_handler::GuiHandler, pi_scope::PiScopeConnector};

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[message(result = "()")]
pub(crate) enum AppEvent{
    UpdateGnss(GnssData),
    UpdateEueler( EulerAngle),
    UpdateQuaternion(Quaternion),
    ServerList(Vec<PiScopeServer>),
    DisconnectOpenPiScope,
    ConnectOpenPiScope(String),
    PiScopeConnected(Option<String>),


}


#[message(result = "()")]
pub struct Subscribe {
    pub recipient: Subscription,
}

pub struct EventBus {
    subscribers: Vec<Subscription>,
}

impl EventBus {
    pub(crate) fn new()->Self{
        Self { subscribers: Vec::new() }
    }
}


#[derive( Clone)]
pub(crate) enum Subscription{
    PiScope(Addr<PiScopeConnector>),
    GuiHandler(Addr<GuiHandler>),
}

impl Subscription {
    async fn call(&self,event: AppEvent)->anyhow::Result<()>{
        match self {
            Subscription::PiScope(addr) => addr.call(event).await?,
            Subscription::GuiHandler(addr) => addr.call(event).await?,
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Actor for EventBus {}

#[async_trait::async_trait]
impl Handler<Subscribe> for EventBus {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Subscribe) {
        self.subscribers.push(msg.recipient);
    }
}

#[async_trait::async_trait]
impl Handler<AppEvent> for EventBus {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: AppEvent) {
        for sub in &self.subscribers {
            let _ = sub.call(msg.clone()).await;
        }
    }
}