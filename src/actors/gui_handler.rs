use slint::{ComponentHandle, SharedString, Weak};
use xactor::{Actor, Addr, Context, Handler};

use crate::{MainWindow, PiScopeLogic, signals::InitGui};

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
                let _ = self.window.upgrade_in_event_loop(|window| {
                    window
                        .set_known_pi_scopes(std::rc::Rc::new(slint::VecModel::from(hosts)).into())
                });
            },
            AppEvent::PiScopeConnected(host) => {
                let _ = self.window.upgrade_in_event_loop(move |window| {
                    let connection=match host {
                        Some(_) =>true,
                        None =>false,
                    };
                    window.global::<PiScopeLogic>().set_connected(connection);
                });
            },
            _ => (),
        }
    }
}

#[async_trait::async_trait]
impl Handler<InitGui> for GuiHandler {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _: InitGui) {
        let bus = self.bus.clone();
        let _ = self.window.upgrade_in_event_loop(move |window| {
            let callback_bus = bus.clone();
            window
                .global::<PiScopeLogic>()
                .on_ConnectPiScopeHost(move |host| {
                    println!("Connect");
                    let _ = callback_bus.send(AppEvent::ConnectOpenPiScope(host.clone().into()));
                });
            let callback_bus = bus.clone();
            window
                .global::<PiScopeLogic>()
                .on_DisconnectPiScopeHost (move || {
                    println!("Connect");
                    let _ = callback_bus.send(AppEvent::DisconnectOpenPiScope);
                });
        });
    }
}
