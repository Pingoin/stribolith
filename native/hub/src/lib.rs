//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod signals;
pub(crate) mod generated{
    pub(crate) mod open_pi_scope;
}

use actors::create_actors;
use rinf::{dart_shutdown, debug_print, write_interface};

use xactor::*;
write_interface!();

#[xactor::main]
async fn main() {
    spawn(create_actors());

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
}