use macroquad::input::KeyCode;
use super::Game;
use macroquad::input::is_key_down;
use macroquad::input::is_key_pressed;

type Key = [Option<KeyCode>; 2];

fn keybind(a: KeyCode, b: KeyCode) -> Key {
    [Some(a), Some(b)]
}

pub struct KeyBind {
    pub keys: Key,
    pub action: fn(&mut Game),
    keytype: KeyType
}

pub struct KeyBinds {
    pub binds: [KeyBind; 4], // increase when more keys are added
}

enum KeyType {
    Hold,
    Press,
}

impl KeyBinds {
    pub fn new() -> Self {
        Self {
            binds: [
                KeyBind {
                    keys: keybind(KeyCode::A, KeyCode::Left),
                    action: Self::action_left,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    keys: keybind(KeyCode::D, KeyCode::Right),
                    action: Self::action_right,
                    keytype: KeyType::Hold,

                },
                KeyBind {
                    keys: keybind(KeyCode::W, KeyCode::Up),
                    action: Self::action_jump,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    keys: [Some(KeyCode::Space), None],
                    action: Self::action_dash,
                    keytype: KeyType::Press,
                },
            ],
        }
    }

    pub fn do_input(&self, game: &mut Game) {
        for kb in &self.binds {
            // to seperate the two types
            let held = match &kb.keytype {
                KeyType::Hold => kb.keys.into_iter().any(|k| k.is_some_and(is_key_down)),
                KeyType::Press => kb.keys.into_iter().any(|k| k.is_some_and(is_key_pressed)),
            };
            if held {
                (kb.action)(game);
            }
        }

    }

    fn action_left(g: &mut Game) {
        g.player.x -= g.player.speed * g.delta_time;
    }

    fn action_right(g: &mut Game) {
        g.player.x += g.player.speed * g.delta_time;
    }

    fn action_jump(g: &mut Game) {
        g.player.y -= g.player.jump * g.delta_time;
    }

    fn action_dash(g: &mut Game) {
        g.player.x += g.player.dash * g.delta_time;
    }
}

