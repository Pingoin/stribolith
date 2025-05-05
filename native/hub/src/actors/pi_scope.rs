use std::{sync::Arc, time::Duration};

use chrono::Utc;
use dashmap::DashMap;
use prost::Message;
use rinf::{DartSignal, RustSignal, debug_print};
use tokio::net::UdpSocket;
use tonic::transport::Channel;
use xactor::*;

use crate::{
    generated::open_pi_scope::{
        open_pi_scope_server_client::OpenPiScopeServerClient, Broadcast, Constants, GnssDataRequest, OrientationDataRequest
    }, signals::{ConnectOpenPiScope, DisconnectOpenPiScope, PiScopeConnected, PiScopeServer, PiScopeServerList}, MutexBox
};

const REQUEST_TIMEOUT: Duration = Duration::from_millis(100);
/// Define message
#[message(result = "bool")]
pub struct Ping;
pub struct PiScopeConnector {
    connection: MutexBox<OpenPiScopeServerClient<Channel>>,
    pi_scopes: Arc<DashMap<String,PiScopeServer>>,
}

impl PiScopeConnector {
    pub fn new() -> PiScopeConnector {
        PiScopeConnector {
            connection: MutexBox::new(),
            pi_scopes:Arc::new(DashMap::new()),
        }
    }
    async fn connect(&self, host: String) -> anyhow::Result<()> {
        let client = OpenPiScopeServerClient::connect(format!("http://{}:50051", host)).await?;

        self.connection.set(Some(client)).await;

        PiScopeConnected{
            host:Some(host),
        }.send_signal_to_dart();
        Ok(())
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        self.connection.set(None).await;

        PiScopeConnected{
            host:None,
        }.send_signal_to_dart();
        Ok(())
    }
}

// Provide Actor implementation for our actor
#[async_trait::async_trait]
impl Actor for PiScopeConnector {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        debug_print!("OpenPiScopeConnector Started");

        let addr = ctx.address().clone();
        spawn(async move {
            let receiver = ConnectOpenPiScope::get_dart_signal_receiver();
            while let Some(signal_pack) = receiver.recv().await {
                let message: ConnectOpenPiScope = signal_pack.message;
                let _ = addr.call(message).await;
            }
        });
        let addr = ctx.address().clone();
        spawn(async move {
            let receiver = DisconnectOpenPiScope::get_dart_signal_receiver();
            while let Some(signal_pack) = receiver.recv().await {
                let message: DisconnectOpenPiScope = signal_pack.message;
                let _ = addr.call(message).await;
            }
        });
        let map_cloned = Arc::clone(&self.pi_scopes);
        spawn(async move {
            let socket = UdpSocket::bind("0.0.0.0:12961").await.unwrap();
            loop {
                let mut buf = [0u8; 1024];
                let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
                match Broadcast::decode(&buf[..len]){
                    Ok(broadcast) => {
                        //debug_print!("Daten Empfangen von {}: {:?}", addr.ip(), &broadcast.magic_number);
                        if broadcast.magic_number == Constants::MagicNumber as u32{
                           let server= PiScopeServer{ host: addr.ip().to_string(), last_telegram:  Utc::now().timestamp_millis()};
                           map_cloned.insert(server.host.clone(), server);
                        }
                    },
                    Err(_) => debug_print!("Mumpitz Empfangen von {}: {:?}", addr.ip(), &buf[..len]),
                }
            }
        });

        let map_cloned = Arc::clone(&self.pi_scopes);
        spawn(async move{


            loop {
                let values: Vec<PiScopeServer> = map_cloned.iter().map(|entry| (*entry).clone()).collect();

                PiScopeServerList{
                    servers:values
                }.send_signal_to_dart();

                sleep(REQUEST_TIMEOUT).await;
            }
        });

        let connection = self.connection.clone_handle();
        spawn(async move {
            loop {
                let gnss = connection
                    .open_async(|mut con| async {
                        match con.get_gnss_data(GnssDataRequest {}).await {
                            Ok(res) => (con, Some(res.into_inner())),
                            Err(_) => (con, None),
                        }
                    })
                    .await.flatten();

                if let Some(gnss) = gnss.map(|gnss| gnss.gnss_data).flatten() {
                    gnss.send_signal_to_dart();
                }
                sleep(REQUEST_TIMEOUT).await;
            }
        });
        let connection = self.connection.clone_handle();
        spawn(async move {
            loop {
                let ins_data = connection
                    .open_async(|mut con| async {
                        match con.get_orientation_data(OrientationDataRequest {}).await {
                            Ok(res) => (con, Some(res.into_inner())),
                            Err(_) => (con, None),
                        }
                    })
                    .await.flatten();

                if let Some(euler) = ins_data.map(|ins| ins.euler).flatten() {
                    euler.send_signal_to_dart();
                }
                if let Some(quaternion) = ins_data.map(|ins| ins.quaternion).flatten() {
                    quaternion.send_signal_to_dart();
                }
                sleep(REQUEST_TIMEOUT).await;
            }
        });

        Ok(())
    }
}

#[async_trait::async_trait]
impl Handler<Ping> for PiScopeConnector {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Ping) -> bool {
        true
    }
}

#[async_trait::async_trait]
impl Handler<ConnectOpenPiScope> for PiScopeConnector {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: ConnectOpenPiScope) {
        debug_print!("connect to: {}", &msg.host);
        match self.connect(msg.host).await {
            Ok(_) => {}
            Err(e) => debug_print!("Error connecting: {:?}", e),
        }
    }
}

#[async_trait::async_trait]
impl Handler<DisconnectOpenPiScope>for PiScopeConnector{
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: DisconnectOpenPiScope) {
        debug_print!("Disonnect");
        match self.disconnect().await {
            Ok(_) => {}
            Err(e) => debug_print!("Error disconnecting: {:?}", e),
        }
    }
}