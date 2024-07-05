use bevy::app::App;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, TryRecvError};

#[derive(Resource, Clone, Debug)]
pub struct CrossbeamEventSender<T: Event + Clone>(pub Sender<T>);

#[derive(Resource, Clone)]
struct CrossbeamEventReceiver<T: Event + Clone>(Receiver<T>);

pub struct MsgBridgePlugin<T: Event + Clone> {
    tx: CrossbeamEventSender<T>,
    rx: CrossbeamEventReceiver<T>,
}

impl<T: Event + Clone> MsgBridgePlugin<T> {
    pub fn new(tx: Sender<T>, rx: Receiver<T>) -> Self {
        Self {
            tx: CrossbeamEventSender(tx),
            rx: CrossbeamEventReceiver(rx),
        }
    }
    fn process_crossbeam_messages(
        receiver: Res<CrossbeamEventReceiver<T>>,
        mut events: EventWriter<T>,
    ) {
        loop {
            match receiver.0.try_recv() {
                Ok(msg) => {
                    events.send(msg);
                }
                Err(TryRecvError::Disconnected) => {
                    panic!("Crossbeam channel disconnected: sender resource dropped")
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
            }
        }
    }
}

impl<T: Event + Clone> Plugin for MsgBridgePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.tx.clone())
            .insert_resource(self.rx.clone())
            .add_event::<T>()
            .add_systems(PreUpdate, Self::process_crossbeam_messages);
    }
}
