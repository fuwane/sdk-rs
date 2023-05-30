//! FuwaNe SDK - Channel

extern crate serde;

use anyhow::Context as AHContext;
use extism_pdk::{ Json, var::{ set, get, remove } };

use anyhow::Error;

pub use fuwane_foundation::binding::Context;

use super::STEREO_FRAME_BYTE_SIZE;


pub struct ContextManager { pub(crate) key: String }

impl ContextManager {
    fn get_impl(&self) -> Result<Option<Json<Context>>, Error> {
        get(&self.key).context("Failed to get context.")
    }

    pub fn get(&self) -> Result<Json<Context>, Error> {
        if let Some(ctx) = self.get_impl()? { Ok(ctx) }
        else { Ok(Json(Context::default())) }
    }

    pub fn set(&mut self, ctx: Json<Context>) -> Result<(), Error> {
        set(&self.key, ctx).context("Failed to set context.")
    }
}

impl Drop for ContextManager {
    fn drop(&mut self) {
        remove(&self.key);
    }
}


pub struct BufferManager { pub(crate) key: String }

impl BufferManager {
    pub fn set(&mut self, data: [u8;STEREO_FRAME_BYTE_SIZE]) -> Result<(), Error> {
        set(&self.key, data.iter().as_slice()).context("Failed to set audio to buffer.")
    }

    pub fn get(&self) -> Result<Option<Vec<u8>>, Error> {
        get(&self.key).context("Failed to get data from audio buffer.")
    }
}

impl Drop for BufferManager {
    fn drop(&mut self) {
        remove(&self.key);
    }
}


pub struct Channel {
    pub id: u64,
    pub id_i64: i64,
    pub ctx: ContextManager,
    pub buffer: BufferManager
}

impl From<u64> for Channel {
    fn from(channel_id: u64) -> Self {
        let channel_string = channel_id.to_string();
        Self {
            id: channel_id, id_i64: channel_id as _,
            ctx: ContextManager { key: format!("{}c", channel_string) },
            buffer: BufferManager { key: format!("{}b", channel_string) }
        }
    }
}