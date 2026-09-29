use super::{BLACK, Object, Rect, TextParams, draw_text_ex};
use crate::FONT;

pub struct Text {
    cbox: Rect,
    text: String,
    font_size: u16,
}

impl Object for Text {
    fn cbox(&self) -> &Rect {
        &self.cbox
    }

    fn draw(&self) {
        let params = TextParams {
            font: Some(FONT.get().unwrap()),
            font_size: self.font_size,
            color: BLACK,
            ..Default::default()
        };
        draw_text_ex(&self.text, self.cbox.x, self.cbox.y, params);
    }
}

impl Text {
    pub async fn new(cbox: Rect, text: &str, font_size: u16) -> Self {
        Self {
            cbox,
            text: text.to_string(),
            font_size,
        }
    }
}
