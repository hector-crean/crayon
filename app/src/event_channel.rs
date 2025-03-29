use bevy::prelude::Event;
use crossbeam_channel::{Receiver, Sender};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use ts_rs::TS;

use crate::event::{CrayonInEvent, CrayonOutEvent};

pub static GLOBAL_EVENT_CHANNEL: Lazy<GlobalEventChannel<CrayonInEvent, CrayonOutEvent>> =
    Lazy::new(GlobalEventChannel::new);

/// A global structure to manage bidirectional communication channels
pub struct GlobalEventChannel<IN, OUT>
where
    IN: Clone + Serialize + for<'a> Deserialize<'a> + Event + TS,
    OUT: Clone + Serialize + for<'a> Deserialize<'a> + Event + TS,
{
    /// Sender for outgoing messages (to be sent from Rust to JavaScript)
    js_tx: Mutex<Option<Sender<IN>>>,

    /// Receiver for incoming messages (to be received by Rust from JavaScript)
    js_rx: Mutex<Option<Receiver<OUT>>>,
}

impl<IN, OUT> Default for GlobalEventChannel<IN, OUT>
where
    IN: Clone + Serialize + for<'a> Deserialize<'a> + Event + TS,
    OUT: Clone + Serialize + for<'a> Deserialize<'a> + Event + TS,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<IN, OUT> GlobalEventChannel<IN, OUT>
where
    IN: Clone + Serialize + for<'a> Deserialize<'a> + Event + TS,
    OUT: Clone + Serialize + for<'a> Deserialize<'a> + Event + TS,
{
    /// Initialize the global event channel
    pub fn new() -> Self {
        Self {
            js_tx: Mutex::new(None),
            js_rx: Mutex::new(None),
        }
    }

    /// Set the sender channel
    pub fn set_sender(&self, sender: Sender<IN>) {
        let mut tx = self.js_tx.lock().expect("Failed to lock tx mutex");
        *tx = Some(sender);
    }

    /// Set the receiver channel
    pub fn set_receiver(&self, receiver: Receiver<OUT>) {
        let mut rx = self.js_rx.lock().expect("Failed to lock rx mutex");
        *rx = Some(receiver);
    }

    /// Send a message through the channel
    pub fn send(&self, event: IN) -> Result<(), String> {
        let tx = self.js_tx.lock().expect("Failed to lock tx mutex");
        match &*tx {
            Some(sender) => sender
                .send(event)
                .map_err(|e| format!("Failed to send event: {}", e)),
            None => Err("Sender channel not initialized".to_string()),
        }
    }

    /// Receive a message from the channel
    pub fn try_receive(&self) -> Result<Option<OUT>, String> {
        let rx = self.js_rx.lock().expect("Failed to lock rx mutex");
        match &*rx {
            Some(receiver) => match receiver.try_recv() {
                Ok(event) => Ok(Some(event)),
                Err(crossbeam_channel::TryRecvError::Empty) => Ok(None),
                Err(e) => Err(format!("Failed to receive event: {}", e)),
            },
            None => Err("Receiver channel not initialized".to_string()),
        }
    }
}
