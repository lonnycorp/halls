use std::sync::{Arc, Mutex};
use std::time::Duration;

use rodio::Source;

use super::data::{TrackData, TrackDataSource};

struct TrackState {
    paused: bool,
    volume: f32,
    dropped: bool,
    generation: u64,
}

pub struct Track {
    data: TrackData,
    state: Arc<Mutex<TrackState>>,
}

pub struct TrackSource {
    data: TrackData,
    state: Arc<Mutex<TrackState>>,
    source: TrackDataSource,
    generation: u64,
    channels: u16,
    sample_rate: u32,
}

impl Track {
    pub fn new(data: TrackData) -> Self {
        return Self {
            data,
            state: Arc::new(Mutex::new(TrackState {
                paused: false,
                volume: 1.0,
                dropped: false,
                generation: 0,
            })),
        };
    }

    pub fn source(&self) -> TrackSource {
        let generation = self.state.lock().unwrap().generation;
        return TrackSource {
            data: self.data.clone(),
            state: Arc::clone(&self.state),
            source: self.data.source(),
            generation,
            channels: self.data.channels(),
            sample_rate: self.data.sample_rate(),
        };
    }

    pub fn set_volume(&self, volume: f32) {
        self.state.lock().unwrap().volume = volume;
    }

    pub fn play(&self) {
        self.state.lock().unwrap().paused = false;
    }

    pub fn pause(&self) {
        self.state.lock().unwrap().paused = true;
    }

    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        state.generation = state.generation.wrapping_add(1);
        state.paused = true;
    }
}

impl Drop for Track {
    fn drop(&mut self) {
        self.state.lock().unwrap().dropped = true;
    }
}

impl Iterator for TrackSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let (dropped, paused, volume, generation) = {
            let state = self.state.lock().unwrap();
            (state.dropped, state.paused, state.volume, state.generation)
        };

        if dropped {
            return None;
        }

        if generation != self.generation {
            self.source = self.data.source();
            self.generation = generation;
        }

        if paused {
            return Some(0.0);
        }

        return match self.source.next() {
            Some(sample) => Some(sample * volume),
            None => Some(0.0),
        };
    }
}

impl Source for TrackSource {
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
        return None;
    }
}
