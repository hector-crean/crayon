use bevy::app::App;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use core::fmt::Debug;

#[derive(Resource, Clone, Debug)]
pub struct ChannelEventSender<T: Event + Clone>(pub Sender<T>);

#[derive(Resource, Clone)]
struct ChannelEventReceiver<T: Event + Clone>(Receiver<T>);

pub struct DuplexPlugin<
    In: Event + Debug + Clone + Send + Sync + 'static, 
    Out: Event + Debug + Clone + Send + Sync + 'static
>  {
    rust_tx: ChannelEventSender<Out>,
    rust_rx: ChannelEventReceiver<In>,
}

impl<In, Out> DuplexPlugin<In, Out> where 
    In: Event + Debug + Clone + Send + Sync + 'static,
    Out: Event + Debug + Clone + Send + Sync + 'static,
{
    pub fn new(rust_tx: Sender<Out>, rust_rx: Receiver<In>) -> Self {
        Self {
            rust_tx: ChannelEventSender(rust_tx),
            rust_rx: ChannelEventReceiver(rust_rx),
        }
    }

    /// This system is called inside Bevy to read from the Crossbeam
    /// receiver of `In` events. If any are present, we insert them
    /// into Bevy’s local event bus (`EventWriter<In>`).
    fn receive_external_event(
        receiver: Res<ChannelEventReceiver<In>>,
        mut event_wtr: EventWriter<In>,
    ) {
        loop {
            match receiver.0.try_recv() {
                Ok(msg) => {
                    info!("Received external message");
                    event_wtr.send(msg);
                }
                Err(TryRecvError::Disconnected) => {
                    error!("Crossbeam channel disconnected: sender resource dropped");
                    break;
                }
                Err(TryRecvError::Empty) => {
                    debug!("No more messages in the channel");
                    break;
                }
            }
        }
    }

     /// This system is called inside Bevy to read local `Out` events,
    /// and if they exist, we send them through the Crossbeam sender
    /// to the outside (JavaScript).
    fn send_events_externally(
        mut events: EventReader<Out>,
        sender: Res<ChannelEventSender<Out>>,
    ) {
        for event in events.read() {
            if let Err(err) = sender.0.send(event.clone()) {
                error!("Failed to send event to external source: {:?}", err);
            } else {
                info!("Event sent to external source: {:?}", event);
              
            }
        }
    }
  
}

impl<In, Out> Plugin for DuplexPlugin<In, Out>
where
    In:  Event + Debug + Clone + Send + Sync + 'static,
    Out: Event + Debug + Clone + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.rust_tx.clone())
            .insert_resource(self.rust_rx.clone())
            .add_event::<In>()
            .add_event::<Out>()
            .add_systems(PreUpdate, Self::receive_external_event)
            .add_systems(PostUpdate, Self::send_events_externally);
    }
}


/// Create a DuplexPlugin plus the complementary Sender/Receiver that
/// JavaScript code should use. In other words:
///
/// - We return a `DuplexPlugin<In, Out>` with `Receiver<In>` and `Sender<Out>`
///   already embedded internally for use by the Bevy engine.
///
/// - We also hand back the other sides of those channels so that your external
///   code can talk to them from outside (like JavaScript).
///
/// If you want your JS side to send `In` events *to* Rust, then from JS you
/// will eventually push into the `Sender<In>` side.  Meanwhile, Rust is
/// holding the `Receiver<In>` side inside the plugin.
///
/// If you want your Rust to send `Out` events *to* JS, then from your
/// Rust Bevy code you send local `Out` events, and the plugin code
/// automatically ships them to the `Sender<Out>`, whose complementary
/// `Receiver<Out>` can be read externally (JS).
pub fn create_duplex_plugin<
    In: Event + Debug + Clone + Send + Sync + 'static, 
    Out: Event + Debug + Clone + Send + Sync + 'static
>() 
    -> (DuplexPlugin<In, Out>, Receiver<Out>, Sender<In>) 
{
    // For In events: external → Rust
    let (js_tx, rust_rx) = crossbeam_channel::unbounded::<In>();
    // For Out events: Rust → external
    let (rust_tx, js_rx) = crossbeam_channel::unbounded::<Out>();

    // The plugin holds the side used by Rust: a receiver for `In` and a sender for `Out`.
    let plugin = DuplexPlugin::new(rust_tx, rust_rx);

    // Meanwhile, we give the caller the complementary ends:
    //   - The sender for `In` so that external code can produce `In` events
    //   - The receiver for `Out` so that external code can read `Out` events
    (plugin, js_rx, js_tx)
}