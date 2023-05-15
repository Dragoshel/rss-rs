use tui::{
    buffer::Buffer,
    layout::{Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::models::channel::Channel;

pub struct ChannelWidget {
    channel: Channel,
}

impl ChannelWidget {
    pub fn new() -> ChannelWidget {
        ChannelWidget {
            channel: Channel::default(),
        }
    }
}

impl Widget for ChannelWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Channel").borders(Borders::ALL);

        let title_span = Spans::from(vec![
            Span::styled("Title:", Style::default().fg(Color::Yellow)),
            Span::from(self.channel.title),
        ]);
        let link_span = Spans::from(vec![
            Span::styled("Link:", Style::default().fg(Color::Yellow)),
            Span::from(self.channel.link),
        ]);
        let description_span = Spans::from(vec![
            Span::styled("Description:", Style::default().fg(Color::Yellow)),
            Span::from(self.channel.description),
        ]);

        let text = Text::from(vec![title_span, link_span, description_span]);

        let paragraph = Paragraph::new(text).block(block);
        paragraph.render(area, buf);
    }
}
