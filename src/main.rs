//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod signals;
pub(crate) mod generated {
    pub(crate) mod open_pi_scope;
}

use actors::{
    event_bus::{EventBus, Subscribe}, gui_handler:: GuiHandler, pi_scope::{self}
};
use anyhow::Ok;
use slint::include_modules;
use xactor::*;

include_modules!();

mod mutex_box;

#[xactor::main]
async fn main() -> anyhow::Result<()> {
    let event_bus_addr = EventBus::new().start().await?;

    let pi_scope_addr = pi_scope::PiScopeConnector::new(event_bus_addr.clone())
        .start()
        .await?;
    event_bus_addr.send(Subscribe {
        recipient: actors::event_bus::Subscription::PiScope(pi_scope_addr.clone()),
    })?;

    let main_window = MainWindow::new()?;

    let main_window_weak = main_window.as_weak();

    let gui_handler_addr= GuiHandler::new(main_window_weak, event_bus_addr.clone()).start().await?;
        event_bus_addr.send(Subscribe {
        recipient: actors::event_bus::Subscription::GuiHandler(gui_handler_addr.clone()),
    })?;

    main_window.run()?;
    Ok(())
}
