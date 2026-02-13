use glam::Vec2;
use winit::keyboard::KeyCode;

const KEY_COUNT: usize = 0x100;

pub enum KeyState {
    Pressed,
    Down,
    Released,
    Up,
}

pub(super) struct InputState {
    mouse_delta: Vec2,
    typed_chars: String,
    tick: u64,
    pressed: [u64; KEY_COUNT],
    released: [u64; KEY_COUNT],
    last_pressed: Option<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        return Self {
            mouse_delta: Vec2::ZERO,
            typed_chars: String::new(),
            tick: 1,
            pressed: [0; KEY_COUNT],
            released: [0; KEY_COUNT],
            last_pressed: None,
        };
    }

    pub fn key(&self, key: KeyCode) -> KeyState {
        let index = key as usize;
        if index >= KEY_COUNT {
            return KeyState::Up;
        }
        let p = self.pressed[index];
        let r = self.released[index];
        if p == self.tick {
            return KeyState::Pressed;
        }
        if r == self.tick {
            return KeyState::Released;
        }
        if p > r {
            return KeyState::Down;
        }
        return KeyState::Up;
    }

    pub fn mouse_delta(&self) -> Vec2 {
        return self.mouse_delta;
    }

    pub fn typed_chars(&self) -> &str {
        return &self.typed_chars;
    }

    pub fn last_pressed(&self) -> Option<KeyCode> {
        return self.last_pressed;
    }

    pub fn handle_key(&mut self, key: KeyCode, pressed: bool, text: Option<&str>) {
        let index = key as usize;
        if index >= KEY_COUNT {
            return;
        }
        if pressed {
            self.pressed[index] = self.tick;
            self.last_pressed = Some(key);
            if let Some(text) = text {
                for c in text.chars().filter(|c| !c.is_control()) {
                    self.typed_chars.push(c);
                }
            }
        } else {
            self.released[index] = self.tick;
        }
    }

    pub fn handle_mouse(&mut self, delta: (f32, f32)) {
        self.mouse_delta += Vec2::new(delta.0, delta.1);
    }

    pub fn reset(&mut self) {
        self.mouse_delta = Vec2::ZERO;
        self.typed_chars.clear();
        self.last_pressed = None;
        self.tick += 1;
    }
}

pub struct InputController<'a> {
    state: &'a mut InputState,
}

impl<'a> InputController<'a> {
    pub(super) fn new(state: &'a mut InputState) -> Self {
        return Self { state };
    }

    pub fn key(&self, key: KeyCode) -> KeyState {
        return self.state.key(key);
    }

    pub fn mouse_delta(&self) -> Vec2 {
        return self.state.mouse_delta();
    }

    pub fn typed_chars(&self) -> &str {
        return self.state.typed_chars();
    }

    pub fn last_pressed(&self) -> Option<KeyCode> {
        return self.state.last_pressed();
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }
}
