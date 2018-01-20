use std::collections::HashSet;

use glutin::{MouseButton, VirtualKeyCode};

#[derive(Debug)]
pub struct KeyState {
    pub down: HashSet<VirtualKeyCode>,
    pub pressed: HashSet<VirtualKeyCode>,
    pub released: HashSet<VirtualKeyCode>,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            down: HashSet::new(),
            pressed: HashSet::new(),
            released: HashSet::new(),
        }
    }

    pub fn from_last_frame(mut old: KeyState) -> Self {
        old.pressed.clear();
        old.released.clear();

        old
    }

    pub fn pressed(&mut self, vk: VirtualKeyCode) {
        self.down.insert(vk);
        self.pressed.insert(vk);
    }

    pub fn released(&mut self, vk: VirtualKeyCode) {
        self.down.remove(&vk);
        self.released.insert(vk);
    }
}

#[derive(Debug)]
pub struct MouseState {
    pub position: (i32, i32),
    pub mouse_wheel_delta: i32,

    pub down: HashSet<MouseButton>,
    pub pressed: HashSet<MouseButton>,
    pub released: HashSet<MouseButton>,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            position: (0, 0),
            mouse_wheel_delta: 0,
            down: HashSet::new(),
            pressed: HashSet::new(),
            released: HashSet::new(),
        }
    }

    pub fn from_last_frame(mut old: MouseState) -> Self {
        old.position = (0, 0);
        old.mouse_wheel_delta = 0;
        old.pressed.clear();
        old.released.clear();

        old
    }

    pub fn pressed(&mut self, vk: MouseButton) {
        self.down.insert(vk);
        self.pressed.insert(vk);
    }

    pub fn released(&mut self, vk: MouseButton) {
        self.down.remove(&vk);
        self.released.insert(vk);
    }
}
