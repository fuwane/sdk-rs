//! FuwaNe SDK - Event

use once_cell::sync::Lazy;

use fuwane_foundation::communication::Channel;


#[derive(Clone, Debug)]
pub enum ChannelEvent { Add(u64), Remove(u64) }

#[derive(Clone, Debug)]
pub enum Event {
    Channel(ChannelEvent)
}


pub static EVENT_CHANNEL: Lazy<Channel<Event>> = Lazy::new(Channel::<Event>::new);