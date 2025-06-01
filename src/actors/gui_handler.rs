use slint::{SharedString, Weak};
use xactor::{Actor, Addr, Context, Handler};

use crate::{MainWindow, signals::InitGui};

use super::event_bus::{AppEvent, EventBus};

#[derive(Clone)]
pub struct GuiHandler {
    window: Weak<MainWindow>,
    bus: Addr<EventBus>,
}

impl GuiHandler {
    pub(crate) fn new(window: Weak<MainWindow>, bus: Addr<EventBus>) -> Self {
        Self { window, bus }
    }
}

#[async_trait::async_trait]
impl Actor for GuiHandler {}

#[async_trait::async_trait]
impl Handler<AppEvent> for GuiHandler {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: AppEvent) {
        match msg {
            AppEvent::ServerList(data) => {
                let hosts: Vec<SharedString> =
                    data.iter().map(|serv| serv.host.clone().into()).collect();
                //self.window.upgrade_in_event_loop(|window| window.set_known_pi_scopes(std::rc::Rc::new(slint::VecModel::from(hosts)).into()));
            },
            _ => (),
        }
    }
}

#[async_trait::async_trait]
impl Handler<InitGui> for GuiHandler {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _: InitGui) {}
}
