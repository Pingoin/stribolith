use std::time::Duration;

use rinf::{debug_print, DartSignal, RustSignal};
use tonic::transport::Channel;
use xactor::*;

use crate::{
    generated::open_pi_scope::{
        open_pi_scope_server_client::OpenPiScopeServerClient, GnssDataRequest, OrientationDataRequest
    }, signals::ConnectOpenPiScope, MutexBox
};

const REQUEST_TIMEOUT:Duration=Duration::from_millis(100);
/// Define message
#[message(result = "bool")]
pub struct Ping;
pub struct PiScopeConnector {
    connection: MutexBox<OpenPiScopeServerClient<Channel>>,
}

impl PiScopeConnector {
    pub fn new() -> PiScopeConnector {
        PiScopeConnector {
            connection: MutexBox::new(),
        }
    }
    async fn connect(&self, host: String) -> anyhow::Result<()> {
        let client = OpenPiScopeServerClient::connect(format!("http://{}:50051", host)).await?;

        self.connection.set(Some(client)).await;

        Ok(())
    }
}

// Provide Actor implementation for our actor
#[async_trait::async_trait]
impl Actor for PiScopeConnector {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        println!("OpenPiScopeConnector Started");

        let addr = ctx.address().clone();
        spawn(async move {
            let receiver = ConnectOpenPiScope::get_dart_signal_receiver();
            while let Some(signal_pack) = receiver.recv().await {
                let message: ConnectOpenPiScope = signal_pack.message;
                let _ = addr.call(message).await;
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
                    .await;

                if let Some(gnss) = gnss.map(|gnss| gnss.gnss_data).flatten(){
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
                .await;

                if let Some(euler) = ins_data.map(|ins|ins.euler).flatten() {
                    euler.send_signal_to_dart();
                }
                if let Some(quaternion) = ins_data.map(|ins|ins.quaternion).flatten() {
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
        debug_print!("connect to: {}",&msg.host);
        match self.connect(msg.host).await  {
            Ok(_) => {},
            Err(e) => debug_print!("Error connecting: {:?}", e),
        }
    }
}
