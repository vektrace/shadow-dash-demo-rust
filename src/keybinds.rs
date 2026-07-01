use super::Game;
use super::player::Player;
use macroquad::input::KeyCode;
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
    keytype: KeyType,
}

pub struct KeyBinds {
    pub binds: [KeyBind; 5], // increase when more keys are added
}

enum KeyType {
    Hold,
    Press,
}

#[derive(PartialEq)]
pub enum KeyName {
    Left,
    Right,
    Jump,
    Dash,
    DebugFly,
}

impl KeyBinds {
    pub fn new() -> Self {
        Self {
            binds: [
                KeyBind {
                    name: KeyName::Left,
                    keys: keybind(KeyCode::A, KeyCode::Left),
                    action: Self::player_left,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    name: KeyName::Right,
                    keys: keybind(KeyCode::D, KeyCode::Right),
                    action: Self::player_right,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    name: KeyName::Jump,
                    keys: keybind(KeyCode::W, KeyCode::Up),
                    action: Self::player_jump,
                    keytype: KeyType::Hold,
                },
                KeyBind {
                    name: KeyName::Dash,
                    keys: [Some(KeyCode::Space), None],
                    action: Self::player_dash,
                    keytype: KeyType::Press,
                },
                KeyBind {
                    name: KeyName::DebugFly,
                    keys: [Some(KeyCode::F), Some(KeyCode::J)],
                    action: Self::player_debug_fly,
                    keytype: KeyType::Hold,
                },
            ],
        }
    }

    pub fn do_input(&self, game: &mut Game) {
        game.player.direction = 0.;
        for kb in &self.binds {
            // to seperate the two types
            let held: bool = match &kb.keytype {
                KeyType::Hold => kb
                    .keys
                    .into_iter()
                    .any(|k: Option<KeyCode>| k.is_some_and(is_key_down)),
                KeyType::Press => kb
                    .keys
                    .into_iter()
                    .any(|k: Option<KeyCode>| k.is_some_and(is_key_pressed)),
            };
            // temp fix so dash works (after dash)
            if kb.name == KeyName::Dash {
                KeyBinds::player_apply_direction(game);
            }
            if held {
                (kb.action)(game);
            }
        }
    }

    fn player_apply_direction(g: &mut Game) {
        g.player.speed.x = g.player.direction * Player::SPEED;
    }

    fn player_left(g: &mut Game) {
        g.player.direction = -1.;
    }

    fn player_right(g: &mut Game) {
        g.player.direction = 1.;
    }

    fn player_jump(g: &mut Game) {
        if g.player.can_jump {
            g.player.is_jumping = true;
            g.player.speed.y = -Player::JUMP;
        }
    }

    fn player_dash(g: &mut Game) {
        g.player.speed.x = g.player.direction * Player::DASH;
        g.player.speed.y = 0.;
    }

    fn player_debug_fly(g: &mut Game) {
        g.player.is_jumping = true;
        if g.player.speed.y > 0. {
            g.player.speed.y = 0.;
        }
        g.player.speed.y += -Player::JUMP / 12.;
    }
}
