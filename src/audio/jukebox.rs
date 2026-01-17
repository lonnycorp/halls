use rodio::dynamic_mixer::DynamicMixerController;
use url::Url;

use super::{Track, TrackData};
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::player::Player;

const RAMP_SPEED: f32 = 0.05;
const MIXER_CHANNELS: u16 = 2;
const MIXER_SAMPLE_RATE: u32 = 44100;

pub struct Jukebox {
    mixer: std::sync::Arc<DynamicMixerController<f32>>,
    tracks: [Option<Track>; 2],
    current: Option<usize>,
    volumes: [f32; 2],
    level_url: Option<Url>,
}

impl Jukebox {
    pub fn new(mixer: &DynamicMixerController<f32>) -> Self {
        let (local_mixer, local_source) =
            rodio::dynamic_mixer::mixer::<f32>(MIXER_CHANNELS, MIXER_SAMPLE_RATE);
        mixer.add(local_source);

        return Self {
            mixer: local_mixer,
            tracks: [None, None],
            current: None,
            volumes: [0.0, 0.0],
            level_url: None,
        };
    }

    fn fade_in(&mut self, track_data: TrackData) {
        let next = match self.current {
            Some(current) => (current + 1) % 2,
            None => 0,
        };

        let track = Track::new(track_data);
        track.set_volume(0.0);
        track.play();
        self.mixer.add(track.source());

        self.tracks[next] = Some(track);
        self.volumes[next] = 0.0;
        self.current = Some(next);
    }

    fn fade_out(&mut self) {
        self.current = None;
    }

    pub fn update(&mut self, player: &Player, cache: &mut LevelCache) {
        if self.level_url.as_ref() != player.level_url() {
            match player.level_url() {
                Some(player_url) => {
                    let player_url = player_url.clone();
                    if let LevelCacheResult::Ready(level) = cache.get(&player_url) {
                        match level.track() {
                            Some(track) => self.fade_in(track.clone()),
                            None => self.fade_out(),
                        }
                        self.level_url = Some(player_url);
                    }
                }
                None => {
                    self.fade_out();
                    self.level_url = None;
                }
            }
        }

        for i in 0..2 {
            let target = if self.current == Some(i) { 1.0 } else { 0.0 };
            let delta = (target - self.volumes[i]).clamp(-RAMP_SPEED, RAMP_SPEED);
            self.volumes[i] += delta;

            if let Some(track) = self.tracks[i].as_ref() {
                track.set_volume(self.volumes[i]);
            }

            if self.current != Some(i) && self.volumes[i] <= 0.0 {
                self.tracks[i] = None;
            }
        }
    }
}
