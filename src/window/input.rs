use glam::Vec2;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::Key;

use super::state::{WindowInputEdge, WindowInputKey, WindowState};

pub enum WindowKeyState {
    Pressed,
    Down,
    Released,
    Up,
}

pub fn window_input_key(state: &WindowState, key: &Key) -> WindowKeyState {
    let Some(key_state) = state.input_keys.get(key) else {
        return WindowKeyState::Up;
    };

    if key_state.tick == state.input_tick {
        if matches!(key_state.edge, WindowInputEdge::Pressed) {
            return WindowKeyState::Pressed;
        }
        return WindowKeyState::Released;
    }

    if matches!(key_state.edge, WindowInputEdge::Pressed) {
        return WindowKeyState::Down;
    }

    return WindowKeyState::Up;
}

pub fn window_input_mouse_delta(state: &WindowState) -> Vec2 {
    return state.input_mouse_delta;
}

pub fn window_input_typed_chars(state: &WindowState) -> &str {
    return &state.input_typed_chars;
}

pub fn window_input_last_pressed(state: &WindowState) -> Option<Key> {
    return state.input_last_pressed.clone();
}

pub fn window_input_key_handle(state: &mut WindowState, event: &KeyEvent) {
    let key = event.logical_key.clone();

    if event.state == ElementState::Pressed {
        state.input_keys.insert(
            key.clone(),
            WindowInputKey {
                tick: state.input_tick,
                edge: WindowInputEdge::Pressed,
            },
        );
        state.input_last_pressed = Some(key);
        if let Some(text) = event.text.as_ref() {
            for c in text.chars().filter(|c| !c.is_control()) {
                state.input_typed_chars.push(c);
            }
        }
        return;
    }

    state.input_keys.insert(
        key,
        WindowInputKey {
            tick: state.input_tick,
            edge: WindowInputEdge::Released,
        },
    );
}

pub fn window_input_mouse_handle(state: &mut WindowState, delta: (f32, f32)) {
    state.input_mouse_delta += Vec2::new(delta.0, delta.1);
}

pub fn window_input_reset(state: &mut WindowState) {
    state.input_mouse_delta = Vec2::ZERO;
    state.input_typed_chars.clear();
    state.input_last_pressed = None;
    state.input_tick += 1;
    state
        .input_keys
        .retain(|_, key_state| matches!(key_state.edge, WindowInputEdge::Pressed));
}
