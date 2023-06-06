use std::{ future::Future, collections::HashMap };

use tokio::{runtime::{ Runtime, Builder }, spawn};

pub mod binding;
pub mod channel;
pub mod event;

use channel::Channel;
use event::{ Event, EVENT_CHANNEL };


pub const SAMPLE_RATE: usize = 48000;
pub const AUDIO_FRAME_RATE: usize = 50;
pub const MONO_FRAME_SIZE: usize = SAMPLE_RATE / AUDIO_FRAME_RATE;
pub const STEREO_FRAME_SIZE: usize = 2 * MONO_FRAME_SIZE;
pub const STEREO_FRAME_BYTE_SIZE: usize = STEREO_FRAME_SIZE * std::mem::size_of::<f32>();


pub struct Service {
    pub channels: HashMap<u64, Channel>
}

impl Service {
    pub fn new() -> Self {
        Self { channels: HashMap::new() }
    }

    pub async fn event_loop(&self) {}

    pub async fn handle_event(&self, event: Event) -> Event {
        event
    }

    pub fn run_with_runtime<F>(
        &self, rt: Runtime,
        mut event_handler: impl FnMut(Event) -> F + Send
    ) where F: Future<Output = ()> + Send + 'static {
        rt.block_on(async move {
            while let Some(mut event) = EVENT_CHANNEL.rx.lock().await.recv().await {
                event = self.handle_event(event).await;
                spawn(event_handler(event));
            };
        });
    }

    pub fn run<F>(&self, event_handler: impl FnMut(Event) -> F + Send)
    where F: Future<Output = ()> + Send + 'static {
        self.run_with_runtime(
            Builder::new_current_thread().build().unwrap(),
            event_handler
        );
    }
}


pub mod prelude {
    pub use extism_pdk::{ *, plugin_fn as entry_point };
    pub extern crate extism_pdk;
}