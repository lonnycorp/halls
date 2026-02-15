use rodio::dynamic_mixer::{DynamicMixer, DynamicMixerController};
use url::Url;

use super::{Track, TrackData};
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::player::Player;

const RAMP_SPEED: f32 = 0.05;
const MIXER_CHANNELS: u16 = 2;
const MIXER_SAMPLE_RATE: u32 = 44100;

pub struct CrossFader {
    mixer: std::sync::Arc<DynamicMixerController<f32>>,
    source: Option<DynamicMixer<f32>>,
    tracks: [Option<Track>; 2],
    current: Option<usize>,
    volumes: [f32; 2],
    level_url: Option<Url>,
}

impl CrossFader {
    pub fn new() -> Self {
        let (local_mixer, local_source) =
            rodio::dynamic_mixer::mixer::<f32>(MIXER_CHANNELS, MIXER_SAMPLE_RATE);

        return Self {
            mixer: local_mixer,
            source: Some(local_source),
            tracks: [None, None],
            current: None,
            volumes: [0.0, 0.0],
            level_url: None,
        };
    }

    pub fn source(&mut self) -> DynamicMixer<f32> {
        return self.source.take().unwrap();
    }

    fn track_fade_in(&mut self, track_data: TrackData) {
        let next = match self.current {
            Some(current) => (current + 1) % 2,
            None => 0,
        };

        let track = Track::new(track_data);
        track.volume_set(0.0);
        track.play();
        self.mixer.add(track.source());

        self.tracks[next] = Some(track);
        self.volumes[next] = 0.0;
        self.current = Some(next);
    }

    fn track_fade_out(&mut self) {
        self.current = None;
    }

    pub fn update(&mut self, player: &Player, cache: &mut LevelCache) {
        if self.level_url.as_ref() != player.level_url() {
            match player.level_url() {
                Some(player_url) => {
                    let player_url = player_url.clone();
                    if let LevelCacheResult::Ready(level) = cache.get(&player_url) {
                        match level.track() {
                            Some(track) => self.track_fade_in(track.clone()),
                            None => self.track_fade_out(),
                        }
                        self.level_url = Some(player_url);
                    }
                }
                None => {
                    self.track_fade_out();
                    self.level_url = None;
                }
            }
        }

        for i in 0..2 {
            let target = if self.current == Some(i) { 1.0 } else { 0.0 };
            let delta = (target - self.volumes[i]).clamp(-RAMP_SPEED, RAMP_SPEED);
            self.volumes[i] += delta;

            if let Some(track) = self.tracks[i].as_ref() {
                track.volume_set(self.volumes[i]);
            }

            if self.current != Some(i) && self.volumes[i] <= 0.0 {
                self.tracks[i] = None;
            }
        }
    }
}
