use macroquad::input::KeyCode;
use super::Game;

type Key = [Option<KeyCode>; 2];

fn keybind(a: KeyCode, b: KeyCode) -> Key {
    [Some(a), Some(b)]
}

pub struct KeyBind {
    pub keys: Key,
    pub action: fn(&mut Game),
}

pub struct KeyBinds {
    pub binds: [KeyBind; 4], // increase when more keys are added
}

impl KeyBinds {
    pub fn new() -> Self {
        Self {
            binds: [
                KeyBind {
                    keys: keybind(KeyCode::A, KeyCode::Left),
                    action: Self::action_left,
                },
                KeyBind {
                    keys: keybind(KeyCode::D, KeyCode::Right),
                    action: Self::action_right,
                },
                KeyBind {
                    keys: keybind(KeyCode::W, KeyCode::Up),
                    action: Self::action_jump,
                },
                KeyBind {
                    keys: [Some(KeyCode::Space), None],
                    action: Self::action_dash,
                },
            ],
        }
    }

    pub fn action_left(g: &mut Game) {
        g.player.x -= g.player.speed * g.delta_time;
    }

    pub fn action_right(g: &mut Game) {
        g.player.x += g.player.speed * g.delta_time;
    }

    pub fn action_jump(g: &mut Game) {
        g.player.y -= g.player.jump * g.delta_time;
    }

    pub fn action_dash(g: &mut Game) {
        g.player.x += g.player.dash * g.delta_time;
    }
}

