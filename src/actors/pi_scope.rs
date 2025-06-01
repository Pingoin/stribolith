use std::{ sync::Arc, time::Duration};

use chrono::Utc;
use dashmap::DashMap;
use prost::Message;
use std::net::SocketAddr;
use socket2::{Socket, Domain, Type, Protocol};
use tokio::net::UdpSocket;
use tonic::transport::Channel;
use xactor::*;

use crate::{
    generated::open_pi_scope::{
        Broadcast, Constants, GnssDataRequest, OrientationDataRequest,
        open_pi_scope_server_client::OpenPiScopeServerClient,
    },
    mutex_box::MutexBox,
    signals::PiScopeServer,
};

use super::event_bus::{AppEvent, EventBus};

const REQUEST_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Clone)]
pub struct PiScopeConnector {
    bus: Addr<EventBus>,
    connection: MutexBox<OpenPiScopeServerClient<Channel>>,
    pi_scopes: Arc<DashMap<String, PiScopeServer>>,
}

impl PiScopeConnector {
    pub fn new(bus: Addr<EventBus>) -> PiScopeConnector {
        PiScopeConnector {
            bus,
            connection: MutexBox::new(),
            pi_scopes: Arc::new(DashMap::new()),
        }
    }
    async fn connect(&self, host: String) -> anyhow::Result<()> {
        let client = OpenPiScopeServerClient::connect(format!("http://{}:50051", host)).await?;

        self.connection.set(Some(client)).await;
        self.bus.send(AppEvent::PiScopeConnected(Some(host)))?;
        Ok(())
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        self.connection.set(None).await;
        self.bus.send(AppEvent::PiScopeConnected(None))?;
        Ok(())
    }
}

// Provide Actor implementation for our actor
#[async_trait::async_trait]
impl Actor for PiScopeConnector {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        println!("OpenPiScopeConnector Started");
        let map_cloned = Arc::clone(&self.pi_scopes);
        spawn(async move {
            let addr: SocketAddr = "0.0.0.0:12961".parse().unwrap();

            let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
            socket.set_reuse_address(true).unwrap();
            socket.set_broadcast(true).unwrap();
            socket.bind(&addr.into()).unwrap();

            let std_socket: std::net::UdpSocket = socket.into();
            std_socket.set_nonblocking(true).unwrap();
            let socket = UdpSocket::from_std(std_socket).unwrap();
            println!("Listening to port 12961");
            loop {
                let mut buf = [0u8; 1024];

                let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
                match Broadcast::decode(&buf[..len]) {
                    Ok(broadcast) => {
                        if broadcast.magic_number == Constants::MagicNumber as u32 {
                            let server = PiScopeServer {
                                host: addr.ip().to_string(),
                                last_telegram: Utc::now().timestamp_millis(),
                            };
                            map_cloned.insert(server.host.clone(), server);
                        }
                    }
                    Err(_) => println!("Mumpitz Empfangen von {}: {:?}", addr.ip(), &buf[..len]),
                }
            }
        });

        let servers = Arc::clone(&self.pi_scopes);
        let bus = self.bus.clone();
        spawn(async move {
            loop {
                let result: Vec<PiScopeServer> = servers.iter().map(|serv| serv.clone()).collect();
                //let result =vec![PiScopeServer{host:"bla".to_string(),last_telegram:0}];
                let _ = bus.send(AppEvent::ServerList(result));
                sleep(Duration::from_secs(5)).await;
            }
        });

        let connection = self.connection.clone_handle();
        let bus = self.bus.clone();
        spawn(async move {
            loop {
                let gnss = connection
                    .open_async(|mut con| async {
                        match con.get_gnss_data(GnssDataRequest {}).await {
                            Ok(res) => (con, Some(res.into_inner())),
                            Err(_) => (con, None),
                        }
                    })
                    .await
                    .flatten();

                if let Some(gnss) = gnss.map(|gnss| gnss.gnss_data).flatten() {
                    let _ = bus.send(AppEvent::UpdateGnss(gnss));
                }
                sleep(REQUEST_TIMEOUT).await;
            }
        });

        let connection = self.connection.clone_handle();
        let bus = self.bus.clone();
        spawn(async move {
            loop {
                let ins_data = connection
                    .open_async(|mut con| async {
                        match con.get_orientation_data(OrientationDataRequest {}).await {
                            Ok(res) => (con, Some(res.into_inner())),
                            Err(_) => (con, None),
                        }
                    })
                    .await
                    .flatten();

                if let Some(euler) = ins_data.map(|ins| ins.euler).flatten() {
                    let _ = bus.send(AppEvent::UpdateEueler(euler));
                }
                if let Some(quaternion) = ins_data.map(|ins| ins.quaternion).flatten() {
                    let _ = bus.send(AppEvent::UpdateQuaternion(quaternion));
                }
                sleep(REQUEST_TIMEOUT).await;
            }
        });

        Ok(())
    }
}

#[async_trait::async_trait]
impl Handler<AppEvent> for PiScopeConnector {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: AppEvent) {
        match msg {
            AppEvent::DisconnectOpenPiScope => {
                println!("Disonnect");
                match self.disconnect().await {
                    Ok(_) => {}
                    Err(e) => println!("Error disconnecting: {:?}", e),
                }
            }
            AppEvent::ConnectOpenPiScope(host) => {
                println!("connect to: {}", &host);
                match self.connect(host).await {
                    Ok(_) => {}
                    Err(e) => println!("Error connecting: {:?}", e),
                }
            }
            _ => (),
        }
    }
}
