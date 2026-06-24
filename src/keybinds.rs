use macroquad::input::KeyCode;
use super::Game;
use macroquad::input::is_key_down;
use macroquad::input::is_key_pressed;

type Key = [Option<KeyCode>; 2];

fn keybind(a: KeyCode, b: KeyCode) -> Key {
    [Some(a), Some(b)]
}

pub struct KeyBind {
    pub name: KeyName,
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

pub enum KeyName {
    Left,
    Right,
    Jump,
    Dash,
}

impl KeyBinds {
    pub fn new() -> Self {
        Self {
            binds: [
                KeyBind {
                    name: KeyName::Left,
                    keys: keybind(KeyCode::A, KeyCode::Left),
                    action: Self::action_left,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    name: KeyName::Right,
                    keys: keybind(KeyCode::D, KeyCode::Right),
                    action: Self::action_right,
                    keytype: KeyType::Hold,

                },
                KeyBind {
                    name: KeyName::Jump,
                    keys: keybind(KeyCode::W, KeyCode::Up),
                    action: Self::action_jump,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    name: KeyName::Dash,
                    keys: [Some(KeyCode::Space), None],
                    action: Self::action_dash,
                    keytype: KeyType::Press,
                },
            ],
        }
    }

    pub fn do_input(&self, game: &mut Game) {       
        game.player.direction = 0.;
        for kb in &self.binds {
            // to seperate the two types
            let held: bool = match &kb.keytype {
                KeyType::Hold => kb.keys.into_iter().any(|k: Option<KeyCode>| k.is_some_and(is_key_down)),
                KeyType::Press => kb.keys.into_iter().any(|k: Option<KeyCode>| k.is_some_and(is_key_pressed)),
            };
            if held {
                (kb.action)(game);
            }
        }



        KeyBinds::apply_direction(game);
    }

    fn apply_direction(g: &mut Game) {
        g.player.xspeed = g.player.direction * g.player.speed * g.delta_time;
    }

    fn action_left(g: &mut Game) {
        g.player.direction = -1.;
        // g.player.xspeed = -1. * g.player.speed * g.delta_time;
    }

    fn action_right(g: &mut Game) {
        g.player.direction = 1.;
        // g.player.xspeed = g.player.speed * g.delta_time;
    }

    fn action_jump(g: &mut Game) {
        g.player.yspeed = -1. * g.player.jump * g.delta_time;
    }

    fn action_dash(g: &mut Game) {
        g.player.xspeed = g.player.direction * g.player.dash * g.delta_time;
        g.player.yspeed = 0.;
    }
}

