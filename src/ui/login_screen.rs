use iced::{Align, Button, Column, Element, Text, TextInput, button, text_input};

use super::{Message, Theme};

#[derive(Debug, Clone, Default)]
pub struct LoginScreen {
    theme: Theme,
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: String,
    username_state: text_input::State,
    password_state: text_input::State,
    client_id_state: text_input::State,
    client_secret_state: text_input::State,
    login_button: button::State,
}

impl LoginScreen {
    /// Create new instance
    pub fn new(theme: Theme) -> LoginScreen {
        LoginScreen {
            theme,
            ..Default::default()
        }
    }

    // Login screen view
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(16)
            .spacing(16)
            .align_items(Align::Center)
            .push(Text::new("Please Login"))
            .push(
                TextInput::new(
                    &mut self.username_state,
                    "Username",
                    &self.username,
                    Message::UsernameChanged
                )
                .style(self.theme)
                .padding(self.theme.input_padding)
            )
            .push(
                TextInput::new(
                    &mut self.password_state,
                    "Password",
                    &self.password,
                    Message::PasswordChanged
                )
                .style(self.theme)
                .padding(self.theme.input_padding)
                .password()
            )
            .push(
                TextInput::new(
                    &mut self.client_id_state,
                    "Client ID",
                    &self.client_id,
                    Message::ClientIdChanged
                )
                .style(self.theme)
                .padding(self.theme.input_padding)
            )
            .push(
                TextInput::new(
                    &mut self.client_secret_state,
                    "Client Secret",
                    &self.client_secret,
                    Message::ClientSecretChanged
                )
                .style(self.theme)
                .padding(self.theme.input_padding)
            )
            .push(
                Button::new(
                    &mut self.login_button,
                    Text::new("Login"),
                )
                .on_press(Message::Login)
                .style(self.theme)
                .padding(16)
            )
            .into()
    }
}