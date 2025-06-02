use slint::{ComponentHandle, SharedString, Weak};
use xactor::{Actor, Addr, Context, Handler};

use crate::{GnssState, MainWindow, PiScopeLogic, signals::InitGui};

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
            }
            AppEvent::PiScopeConnected(host) => {
                let _ = self.window.upgrade_in_event_loop(move |window| {
                    let connection = match host {
                        Some(_) => true,
                        None => false,
                    };
                    window.global::<PiScopeLogic>().set_connected(connection);
                });
            }
            AppEvent::UpdateGnss(gnss) => {
                let _ = self.window.upgrade_in_event_loop(move |window| {

                    let sats:Vec< crate::Sattelite>= gnss.satellites.iter().map(|s| {
                        crate::Sattelite {
                            azimuth:s.azimuth,
                            elevation:s.elevation,
                            prn:s.prn, 
                            signalStrenght:s.signal_strength, 
                            system: s.system().as_str_name().into(), 
                            used: s.used}
                            
                        
                    }).collect();

                    window.global::<GnssState>().set_gnss(crate::GnssData {
                        alt: gnss.alt,
                        climb: gnss.climb,
                        err_alt: gnss.estimated_error_altitude,
                        err_climb: gnss.estimated_error_climb,
                        err_lat: gnss.estimated_error_latitude,
                        err_lon: gnss.estimated_error_longitude,
                        err_plane: gnss.estimated_error_plane,
                        err_speed: gnss.estimated_error_speed,
                        err_track: gnss.estimated_error_track,
                        lat: gnss.lat as f32,
                        leap_seconds: gnss.leap_seconds,
                        long: gnss.lon as f32,
                        mode: gnss.mode().as_str_name().into(),
                        satellites:sats.as_slice().into(),
                        speed: gnss.speed,
                        track: gnss.track,
                    });
                });
            }
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
                .on_DisconnectPiScopeHost(move || {
                    println!("Connect");
                    let _ = callback_bus.send(AppEvent::DisconnectOpenPiScope);
                });
        });
    }
}
