//! FuwaNe SDK - Event

use std::sync::Arc;

use tokio::sync::{ mpsc::{ Sender, Receiver, channel }, Mutex as AioMutex };
use once_cell::sync::Lazy;


#[derive(Clone, Debug)]
pub enum ChannelEvent { Add(u64), Remove(u64) }

#[derive(Clone, Debug)]
pub enum Event {
    Channel(ChannelEvent)
}


pub struct EventChannel {
    pub tx: Sender<Event>,
    pub(crate) rx: Arc<AioMutex<Receiver<Event>>>
}


pub static EVENT_CHANNEL: Lazy<EventChannel> = Lazy::new(|| {
    let (tx, rx) = channel(128); EventChannel {
        tx: tx, rx: Arc::new(AioMutex::new(rx))
    }
});