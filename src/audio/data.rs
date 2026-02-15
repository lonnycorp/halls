use std::io::Cursor;
use std::sync::Arc;

use rodio::{Decoder, Source};

#[derive(Clone)]
pub struct TrackData {
    samples: Arc<[f32]>,
    channels: u16,
    sample_rate: u32,
    repeat: bool,
}

impl TrackData {
    pub fn new(data: &[u8], repeat: bool) -> Result<Self, rodio::decoder::DecoderError> {
        let cursor = Cursor::new(data.to_vec());
        let decoder = Decoder::new(cursor)?;

        let channels = decoder.channels();
        let sample_rate = decoder.sample_rate();
        let samples: Arc<[f32]> = decoder.convert_samples().collect::<Vec<f32>>().into();

        return Ok(Self {
            samples,
            channels,
            sample_rate,
            repeat,
        });
    }

    pub fn channels(&self) -> u16 {
        return self.channels;
    }

    pub fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    pub(crate) fn samples(&self) -> Arc<[f32]> {
        return Arc::clone(&self.samples);
    }

    pub(crate) fn repeat(&self) -> bool {
        return self.repeat;
    }
}
