//! FuwaNe SDK - Channel

extern crate serde;

use extism_pdk::{ Json, var::{ set, get, remove } };

use super::STEREO_FRAME_BYTE_SIZE;


pub struct Channel {
    pub id: u64,
    pub id_i64: i64,
    pub id_string: String
}


impl Channel {
    pub fn get_context<'a>(&self) -> Json<Context> {
        if let Some(ctx) = get(&self.id_string).unwrap() { ctx }
        else { get(&self.id_string).unwrap().unwrap() }
    }

    pub fn set_context(&mut self, ctx: Json<Context>) {
        set(&self.id_string, ctx).unwrap();
    }

    pub fn set_audio_frame(&mut self, data: [u8;STEREO_FRAME_BYTE_SIZE]) {
        let mut ctx = self.get_context();
        ctx.0.buffer = data;
        self.set_context(ctx);
    }
}

impl From<u64> for Channel {
    fn from(channel_id: u64) -> Self {
        Self {
            id: channel_id, id_i64: channel_id as _,
            id_string: channel_id.to_string()
        }
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        remove(&self.id_string);
    }
}