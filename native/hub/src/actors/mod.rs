//! This module contains actors.
//! To build a solid app, avoid communicating by sharing memory.
//! Focus on message passing instead.

//mod first;
//mod second;
mod pi_scope;
use std::time::Duration;
use tokio::time::sleep;

use xactor::*;
//use first::FirstActor;
use pi_scope::Ping;
use rinf::debug_print;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

/// Creates and spawns the actors in the async system.
pub async fn create_actors()->Result<()> {
    // Though simple async tasks work, using the actor model
    // is highly recommended for state management
    // to achieve modularity and scalability in your app.
    // Actors keep ownership of their state and run in their own loops,
    // handling messages from other actors or external sources,
    // such as websockets or timers.

    let addr = pi_scope::PiScopeConnector{}.start().await?;

    sleep(Duration::from_secs(2)).await;

    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let result = addr.call(Ping {}).await;

    match result {
        Ok(res) => debug_print!("Got result: {}", res),
        Err(err) => debug_print!("Got error: {}", err),
    };

    Ok(())
}
