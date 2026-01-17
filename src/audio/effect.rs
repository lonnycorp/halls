use rodio::dynamic_mixer::DynamicMixerController;

use super::{Track, TrackData};

pub struct Effect {
    track: Track,
}

impl Effect {
    pub fn new(mixer: &DynamicMixerController<f32>, track_data: TrackData) -> Self {
        let track = Track::new(track_data);
        track.pause();
        mixer.add(track.source());
        return Self { track };
    }

    pub fn reset(&self) {
        self.track.reset();
    }

    pub fn play(&self) {
        self.track.play();
    }

    pub fn pause(&self) {
        self.track.pause();
    }
}
