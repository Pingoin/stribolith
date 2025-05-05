//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod signals;
pub(crate) mod generated {
    pub(crate) mod open_pi_scope;
}

use std::sync::Arc;

use actors::create_actors;
use rinf::{dart_shutdown, debug_print, write_interface};

use tokio::sync::{Mutex, Notify};
use xactor::*;
write_interface!();

#[xactor::main]
async fn main() {
    spawn(create_actors());

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
}

pub struct MutexBox<T> {
    inner: Arc<Mutex<Option<T>>>,
    taken: Arc<Mutex<bool>>,
    notify: Arc<Notify>,
}

impl<T> MutexBox<T>
where
    T: Clone,
{
    pub fn new() -> MutexBox<T> {
        Self {
            inner: Arc::new(Mutex::new(None)),
            taken: Arc::new(Mutex::new(false)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Nimmt den Wert temporär heraus, führt async-Funktion aus, setzt ihn wieder ein
    pub async fn take_with<F, Fut, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(T) -> Fut,
        Fut: std::future::Future<Output = (T, R)>,
    {
        let mut lock = self.inner.lock().await;
        let value = lock.take()?;
        let mut taken = self.taken.lock().await;
        *taken = true;
        drop(taken);
        drop(lock); // nicht während Await halten!

        let (new_value, result) = f(value).await;

        let mut lock = self.inner.lock().await;
        *lock = Some(new_value);
        let mut taken = self.taken.lock().await;
        *taken = false;
        drop(taken);
        drop(lock);
        self.notify.notify_one();
        Some(result)
    }

    pub async fn is_taken(&self) -> bool {
        let taken = self.taken.lock().await;
        taken.clone()
    }

    pub async fn open_async<F, Fut, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(T) -> Fut + Clone,
        Fut: std::future::Future<Output = (T, R)>,
    {
        loop {
            if !self.is_taken().await {
                return self.take_with(f.clone()).await;
            }
            self.notify.notified().await;
        }
    }

    pub async fn set(&self, value: Option<T>) {
        loop {
            if !self.is_taken().await {
                let mut lock = self.inner.lock().await;
                *lock = value;
                drop(lock);
                self.notify.notify_one();
                return;
            }
            self.notify.notified().await;
        }
    }

    /// Gibt eine clonbare Referenz mit `'static`-Lifetime zurück.
    pub fn clone_handle(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            taken: Arc::clone(&self.taken),
            notify: Arc::clone(&self.notify),
        }
    }
}
