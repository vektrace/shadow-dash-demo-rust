use macroquad::prelude::*;

mod destroying_platform;
mod extra_dash;
mod level_trigger;
mod moving_platform;
mod platform;
mod spikes;
mod text;
mod trampoline;

pub use destroying_platform::DestroyingPlatform;
pub use extra_dash::ExtraDash;
pub use level_trigger::LevelTrigger;
pub use moving_platform::MovingPlatform;
pub use platform::Platform;
pub use spikes::Spikes;
pub use text::Text;
pub use trampoline::Trampoline;

pub trait Object {
    fn cbox(&self) -> &Rect;
    // fn properties(&self) -> HashMap<&str, &str>;
    fn draw(&self);
}

pub fn draw_all(objects: &Vec<Box<dyn Object>>) {
    for object in objects {
        object.draw();
    }
}
