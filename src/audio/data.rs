use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;

use rodio::{Decoder, Source};

#[derive(Clone)]
pub struct TrackData {
    samples: Arc<Vec<f32>>,
    channels: u16,
    sample_rate: u32,
    repeat: bool,
}

#[derive(Debug, Clone)]
pub enum TrackDataError {
    Load,
}

impl TrackData {
    pub fn new(data: &[u8], repeat: bool) -> Result<Self, TrackDataError> {
        let cursor = Cursor::new(data.to_vec());
        let decoder = Decoder::new(cursor).map_err(|_| TrackDataError::Load)?;

        let channels = decoder.channels();
        let sample_rate = decoder.sample_rate();
        let samples: Vec<f32> = decoder.convert_samples().collect();

        return Ok(Self {
            samples: Arc::new(samples),
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

    pub(crate) fn source(&self) -> TrackDataSource {
        return TrackDataSource {
            samples: Arc::clone(&self.samples),
            channels: self.channels,
            sample_rate: self.sample_rate,
            pos: 0,
            repeat: self.repeat,
        };
    }
}

pub(crate) struct TrackDataSource {
    samples: Arc<Vec<f32>>,
    channels: u16,
    sample_rate: u32,
    pos: usize,
    repeat: bool,
}

impl Iterator for TrackDataSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.pos >= self.samples.len() {
            if !self.repeat {
                return None;
            }
            self.pos = 0;
        }

        let sample = self.samples[self.pos];
        self.pos += 1;
        return Some(sample);
    }
}

impl Source for TrackDataSource {
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn channels(&self) -> u16 {
        return self.channels;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn total_duration(&self) -> Option<Duration> {
        if self.repeat {
            return None;
        }

        let samples_per_channel = self.samples.len() / self.channels as usize;
        let secs = samples_per_channel as f64 / self.sample_rate as f64;
        return Some(Duration::from_secs_f64(secs));
    }
}
