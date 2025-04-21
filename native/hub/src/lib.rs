//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod signals;

use actors::create_actors;
use rinf::{dart_shutdown, debug_print, write_interface, DartSignal};
use signals::MyPreciousData;
use tokio::spawn;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    //spawn(create_actors());
    spawn(calculate_precious_data());

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
}


pub async fn calculate_precious_data() {
    let receiver = MyPreciousData::get_dart_signal_receiver(); // GENERATED
    while let Some(signal_pack) = receiver.recv().await {
      let my_precious_data = signal_pack.message;
  
      let new_numbers: Vec<i32> = my_precious_data
        .input_numbers
        .into_iter()
        .map(|x| x + 1)
        .collect();
      let new_string = my_precious_data.input_string.to_uppercase();
  
      debug_print!("{:?}", new_numbers);
      debug_print!("{}", new_string);
    }
  }