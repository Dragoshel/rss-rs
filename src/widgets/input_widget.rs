use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct InputWidget {
    input: String,
}

impl InputWidget {
    pub fn new(input: &String) -> InputWidget {
        InputWidget {
            input: input.to_owned(),
        }
    }
}

impl Widget for InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Input").borders(Borders::ALL);
        let paragraph = Paragraph::new(self.input).block(block);
        paragraph.render(area, buf);
    }
}
