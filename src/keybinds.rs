use super::Game;
use super::player::{Player, PlayerDashState, PlayerJumpState};
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
    pub binds: [KeyBind; 6], // increase when more keys are added
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
    DebugToggle,
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
                    keytype: KeyType::Press,
                },
                KeyBind {
                    name: KeyName::Dash,
                    keys: [Some(KeyCode::Space), None],
                    action: Self::player_dash,
                    keytype: KeyType::Press,
                },
                KeyBind {
                    name: KeyName::DebugToggle,
                    keys: [Some(KeyCode::F3), None],
                    action: Self::player_debug_toggle,
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
        // don't reset the direction while dashing
        if game.player.dash != PlayerDashState::IsDashing {
            game.player.direction = 0.;
        }
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

            if held
                && (
                    // lock all non-special keybinds while dashing

                    // if you're not dashing or if its a special keybind
                    game.player.dash != PlayerDashState::IsDashing
                        || [KeyName::DebugToggle].contains(&kb.name)
                )
            {
                (kb.action)(game);
            }
        }
    }

    fn player_apply_direction(g: &mut Game) {
        g.player.velocity.x = g.player.direction * Player::SPEED;
    }

    fn player_left(g: &mut Game) {
        g.player.direction -= 1.;
    }

    fn player_right(g: &mut Game) {
        g.player.direction += 1.;
    }

    fn player_jump(g: &mut Game) {
        match g.player.jump {
            PlayerJumpState::CanJump => {
                g.player.jump = PlayerJumpState::CanDoubleJump;

                g.player.velocity.y = -Player::JUMP;
            }
            PlayerJumpState::CanDoubleJump => {
                println!("double jump!");
                g.player.velocity.y = -Player::JUMP;
                g.player.jump = PlayerJumpState::Used;
            }
            PlayerJumpState::Used => {}
        }
    }

    fn player_dash(g: &mut Game) {
        if g.player.dash == PlayerDashState::CanDash {
            // not speed or accell because it tp the player and doesnt
            // accellerate him
            g.player.dash = PlayerDashState::IsDashing;
            g.player.dash_dest = g.player.cbox.x + Player::DASH_DIST * g.player.direction;
            g.player.accell *= 0.;
            g.player.velocity *= 0.;
        }
    }
    fn player_debug_toggle(g: &mut Game) {
        g.debug = !g.debug;
    }

    fn player_debug_fly(g: &mut Game) {
        if !g.debug {
            return;
        }

        if g.player.accell.y > 0. {
            g.player.accell.y = 0.;
            g.player.velocity.y = 0.;
        }
        g.player.on_ground = true;
        g.player.accell.y += -Player::JUMP / 800. * Player::GRAVITY;
    }
}
