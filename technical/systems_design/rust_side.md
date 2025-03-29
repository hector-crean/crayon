
The DuplexPlugin is a Bevy plugin designed to facilitate communication between Rust and an external source (e.g., JavaScript) using crossbeam channels. It handles both sending and receiving events.

```rust

#[derive(Resource, Clone, Debug)]
pub struct ChannelEventSender<T: Event + Clone>(pub Sender<T>);

#[derive(Resource, Clone)]
struct ChannelEventReceiver<T: Event + Clone>(Receiver<T>);

pub struct DuplexPlugin<T: Event + Debug + Clone + Send + Sync + 'static> {
    rust_tx: ChannelEventSender<T>,
    rust_rx: ChannelEventReceiver<T>,
}


// Helper function to create a new DuplexPlugin with channels
pub fn create_duplex_plugin<T: Event + Debug + Clone + Send + Sync + 'static>() -> (DuplexPlugin<T>, Sender<T>, Receiver<T>) {
    // Channel for sending events from JavaScript to Rust
    let (js_tx, rust_rx) = crossbeam_channel::unbounded();

    // Channel for sending events from Rust to JavaScript
    let (rust_tx, js_rx) = crossbeam_channel::unbounded();

    // Create the plugin with the appropriate ends of each channel
    let plugin = DuplexPlugin::new(rust_tx, rust_rx);

    // Return the plugin and the JavaScript-facing ends of both channels
    (plugin, js_tx, js_rx)
}

```



And then we have a global event channel

```rust


pub static GLOBAL_EVENT_CHANNEL: Lazy<GlobalEventChannel> = Lazy::new(GlobalEventChannel::new);

/// A global structure to manage bidirectional communication channels
pub struct GlobalEventChannel {
    /// Sender for outgoing messages (to be sent from Rust to JavaScript)
    js_tx: Mutex<Option<Sender<CrayonEvent>>>,
    
    /// Receiver for incoming messages (to be received by Rust from JavaScript)
    js_rx: Mutex<Option<Receiver<CrayonEvent>>>,
}


```



Alternaitives
1. Message passing (js postmessage API to send messages between the main thread and webassembly modules?)

2. Shared memory

3. Websockets?

4. Event driven architecture


