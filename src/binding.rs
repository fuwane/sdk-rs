use extism_pdk::*;
use serde_json::Value;

use tokio::spawn;

use super::{ EVENT_CHANNEL, Event, event::ChannelEvent };



#[host_fn]
extern "ExtismHost" {
    pub fn play(channel_id_i64: i64, is_stereo: i32);
    pub fn send_audio_data(channel_id_i64: i64) -> (i64, i64);
}


fn dispatch_channel_event(ce: ChannelEvent) {
    spawn(async { EVENT_CHANNEL.tx.send(Event::Channel(ce)).await.unwrap(); });
}

#[plugin_fn]
pub fn create_channel(channel_id: Value) -> FnResult<()> {
    dispatch_channel_event(ChannelEvent::Add(channel_id.as_u64().unwrap()));
    Ok(())
}

#[plugin_fn]
pub fn drop_channel(channel_id: Value) -> FnResult<()> {
    dispatch_channel_event(ChannelEvent::Remove(channel_id.as_u64().unwrap()));
    Ok(())
}