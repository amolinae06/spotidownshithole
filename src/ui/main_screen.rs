use iced::{Align, Button, Column, Container, Element, Length, ProgressBar, Row, Scrollable, Space, Svg, Text, TextInput, button, scrollable, svg, text_input};

use crate::{downloader::{Download, DownloadState}, ui::theme::IconButton};

use super::{Message, Theme};

#[derive(Debug, Default)]
pub struct MainScreen {
    theme: Theme,
    pub downloads: Vec<Download>,

    pub query: String,
    query_state: text_input::State,
    query_submit: button::State,
    scroll_state: scrollable::State,
    settings_state: button::State
}

impl MainScreen {
    /// Create new instance
    pub fn new(theme: Theme) -> MainScreen {
        MainScreen {
            theme,
            ..Default::default()
        }
    }
    
    /// iced view
    pub fn view(&mut self) -> Element<Message> {
        let mut downloads = Scrollable::new(&mut self.scroll_state)
            .spacing(12)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Start);

        for d in &self.downloads {
            // State text
            let mut error = false;
            let (text, progress) = match d.state {
                DownloadState::None => ("0%".to_string(), 0.0),
                DownloadState::Lock => ("0%".to_string(), 0.0),
                DownloadState::Downloading(r, t) => {
                    let p = (r as f32 / t as f32) * 100.0;
                    // Sometimes progress can be above 100% due to size being MP3 encoded size
                    if p > 100.0 {
                        ("POST".to_string(), 100.0)
                    } else {
                        (format!("{}%", p as u64), p)
                    }
                },
                DownloadState::Post => ("POST".to_string(), 100.0),
                DownloadState::Done => ("DONE".to_string(), 100.0),
                DownloadState::Error(_) => {
                    error = true;
                    ("ERR".to_string(), 0.0)
                },
            };

            downloads = downloads.push(
                Column::new()
                .push(Row::new()
                    .spacing(8)
                    .push(
                        Text::new(&text)
                        .size(28)
                        .color(match error {
                            true => self.theme.error,
                            false => self.theme.accent
                        })
                        .width(Length::Units(80))
                    )
                    .push(
                    Column::new()
                        .push(Text::new(&d.title).size(16))
                        .push(Text::new(&d.subtitle).size(14))
                    )
                )
                // Padding
                .push(Space::with_height(Length::Units(4)))
                // Progress
                .push(
                    ProgressBar::new(0.0..=100.0, progress)
                    .style(self.theme)
                    .height(Length::Units(4))
                )
            );
        }

        Column::new()
        .padding(16)
        .spacing(8)
        .align_items(Align::Center)
        .push(
            Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Container::new(
                    Svg::new(
                        svg::Handle::from_memory(include_bytes!("../../assets/banner.svg").to_vec())
                    )
                    .height(Length::Units(56))
                )
                .width(Length::Fill)
            )
            // .push(Space::with_width(Length::FillPortion(2)))
            .push(
                Button::new(
                    &mut self.settings_state,
                    Svg::new(
                        svg::Handle::from_memory(include_bytes!("../../assets/settings.svg").to_vec())
                    )
                )
                .style(IconButton {})
                .on_press(Message::OpenSettings)
                .height(Length::Units(48))
                .padding(8)
            )
        )
        .push(
            Space::with_height(Length::Units(8))
        )
        .push(
            Row::new()
                .spacing(8)
                .push(
                    TextInput::new(
                        &mut self.query_state,
                        "Enter URL or URI",
                        &self.query,
                        Message::QueryChanged
                    )
                    .style(self.theme)
                    .padding(self.theme.input_padding)
                )
                .push(
                    Button::new(
                        &mut self.query_submit,
                        Text::new("Add")
                    )
                    .on_press(Message::QuerySubmit)
                    .style(self.theme)
                    .padding(8)
                )
        )
        .push(Space::with_height(Length::Units(8)))
        .push(Text::new("Downloads:"))
        .push(Space::with_height(Length::Units(8)))
        .push(downloads)
        .into()
    }
}