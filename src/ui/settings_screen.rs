use iced::{Button, Checkbox, Column, Element, Length, PickList, Row, Slider, VerticalAlignment, HorizontalAlignment, Space, Text, TextInput, button, pick_list, slider, text_input};

use crate::{downloader::{DownloaderConfig, Quality}, ui::theme::DarkButton};

use super::{Message, Theme};

pub struct SettingsScreen {
    theme: Theme,
    pub config: DownloaderConfig,

    path: text_input::State,
    filename_template: text_input::State,
    concurrent_downloads: slider::State,
    quality: pick_list::State<Quality>,
    separator: text_input::State,
    discard_button: button::State,
    save_button: button::State
}

impl SettingsScreen {
    pub fn new(theme: Theme, config: DownloaderConfig) -> SettingsScreen {
        SettingsScreen {
            theme, config,
            path: text_input::State::default(),
            filename_template: text_input::State::default(),
            concurrent_downloads: slider::State::default(),
            quality: pick_list::State::default(),
            separator: text_input::State::default(),
            discard_button: button::State::default(),
            save_button: button::State::default(),
        }
    }

    /// iced view
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
        .padding(16)
        .spacing(16)
        .push(
            Text::new("Settings")
            .width(Length::Fill)
            .size(32)
            .horizontal_alignment(HorizontalAlignment::Center)
        )
        .push(Space::with_height(Length::Units(32)))
        .push(
            TextInput::new(
                &mut self.path,
                "Download Path",
                self.config.path.to_str().unwrap(),
                Message::SetDownloadPath
            )
            .style(self.theme)
            .padding(self.theme.input_padding)
        )
        .push(
            TextInput::new(
                &mut self.filename_template,
                "Filename Template",
                &self.config.filename_template,
                Message::SetFilenameTemplate
            )
            .style(self.theme)
            .padding(self.theme.input_padding)
        )
        .push(
            Checkbox::new(
                self.config.convert_to_mp3,
                "Convert to MP3",
                Message::SetConvertToMP3
            )
            .style(self.theme)
        )
        .push(
            Row::new()
            .push(
                Text::new(format!("Concurrent Downloads: {}", self.config.concurrent_downloads))
            )
            .push(Space::with_width(Length::Units(16)))
            .push(
                Slider::new(
                    &mut self.concurrent_downloads,
                    1..=8,
                    self.config.concurrent_downloads as i32,
                    Message::SetConcurrentDownloads
                )
                .style(self.theme)
            )
        )
        .push(
            Row::new()
            .push(
                Text::new("Quality: ")
                .height(Length::Fill)
                .vertical_alignment(VerticalAlignment::Center)
            )
            .push(
                PickList::new(
                    &mut self.quality,
                    &Quality::ALL[..],
                    Some(self.config.quality),
                    Message::SetQuality
                )
                .style(self.theme)
                .padding(self.theme.input_padding)
            )
            .height(Length::Units(28))
        )
        .push(
            TextInput::new(
                &mut self.separator,
                "Tag Separator",
                &self.config.separator,
                Message::SetSeparator
            )
            .style(self.theme)
            .padding(self.theme.input_padding)
        )
        .push(
            Checkbox::new(
                self.config.id3v24,
                "Use ID3 v2.4 Tag for MP3",
                Message::SetID3v24
            )
            .style(self.theme)
        )
        .push(
            Space::with_height(Length::Units(16))
        )
        .push(
            Row::new()
            .width(Length::Fill)
            .push(
                Button::new(
                    &mut self.discard_button,
                    Text::new("Discard").horizontal_alignment(HorizontalAlignment::Center)
                )
                .on_press(Message::DiscardSettings)
                .style(DarkButton {})
                .width(Length::Fill)
            )
            .push(Space::with_width(Length::Units(16)))
            .push(
                Button::new(
                    &mut self.save_button,
                    Text::new("Save").horizontal_alignment(HorizontalAlignment::Center)
                )
                .on_press(Message::SaveSettings)
                .style(self.theme)
                .width(Length::Fill)
            )
        )
        .into()
    }
}