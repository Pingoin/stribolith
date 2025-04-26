use std::time::Duration;

use rinf::{DartSignal, RustSignal, debug_print};
use xactor::*;

use crate::{
    generated::open_pi_scope::{
        GnssDataRequest, open_pi_scope_server_client::OpenPiScopeServerClient,
    },
    signals::MyPreciousData,
};

/// Define message
#[message(result = "bool")]
pub struct Ping;
pub struct PiScopeConnector {}

impl PiScopeConnector {}

// Provide Actor implementation for our actor
#[async_trait::async_trait]
impl Actor for PiScopeConnector {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        println!("OpenPiScopeConnector Parent Started");
        // Nebenläufig Daten vom Server laden
        let addr = ctx.address().clone();
        spawn(async move {
            let receiver = MyPreciousData::get_dart_signal_receiver();
            while let Some(signal_pack) = receiver.recv().await {
                let message: MyPreciousData = signal_pack.message;
                let _ = addr.call(message).await;
            }
        });

        spawn(async move {
            loop {
                let mut client = OpenPiScopeServerClient::connect("http://192.168.178.84:50051")
                    .await
                    .unwrap();
                let gnss = client.get_gnss_data(GnssDataRequest {}).await.unwrap();
                debug_print!("{:?}", &gnss);
                if let Some(gnss) = gnss.into_inner().gnss_data {
                    gnss.send_signal_to_dart();
                }
                sleep(Duration::from_secs(1)).await;
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
impl Handler<MyPreciousData> for PiScopeConnector {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: MyPreciousData) {
        debug_print!("message received: {:?}", msg);
    }
}
