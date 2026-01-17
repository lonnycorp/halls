use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use lru::LruCache;
use url::Url;

use super::error::LevelLoadError;
use super::Level;

enum LevelEntry {
    Loading(Option<JoinHandle<Result<Arc<Level>, LevelLoadError>>>),
    Ready(Arc<Level>),
    Failed(LevelLoadError),
}

pub enum LevelCacheResult {
    Loading,
    Ready(Arc<Level>),
    Failed(LevelLoadError),
}

pub struct LevelCache {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    pending: VecDeque<Url>,
    cache: LruCache<Url, LevelEntry>,
}

impl LevelCache {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>, capacity: usize) -> Self {
        Self {
            device,
            queue,
            pending: VecDeque::new(),
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    pub fn get(&mut self, url: &Url) -> LevelCacheResult {
        if self.cache.contains(url) {
            return match self.cache.get(url).unwrap() {
                LevelEntry::Ready(level) => LevelCacheResult::Ready(Arc::clone(level)),
                LevelEntry::Loading(_) => LevelCacheResult::Loading,
                LevelEntry::Failed(err) => LevelCacheResult::Failed(err.clone()),
            };
        }

        let device = Arc::clone(&self.device);
        let queue = Arc::clone(&self.queue);
        let url_clone = url.clone();
        let handle = thread::spawn(move || Level::new(url_clone, &device, &queue).map(Arc::new));

        self.cache
            .put(url.clone(), LevelEntry::Loading(Some(handle)));
        self.pending.push_back(url.clone());

        return LevelCacheResult::Loading;
    }

    pub fn update(&mut self) {
        let Some(url) = self.pending.front().cloned() else {
            return;
        };
        let Some(entry) = self.cache.get_mut(&url) else {
            return;
        };
        let LevelEntry::Loading(handle_opt) = entry else {
            panic!("pending queue contains URL with Ready entry");
        };

        if handle_opt.as_ref().is_some_and(|h| h.is_finished()) {
            let handle = handle_opt.take().unwrap();
            match handle.join().unwrap() {
                Ok(level) => {
                    *entry = LevelEntry::Ready(level);
                }
                Err(error) => {
                    *entry = LevelEntry::Failed(error);
                }
            }
            self.pending.pop_front();
        }
    }
}
