use std::path::Path;
use std::time::Duration;
use iced::window::Icon;
use iced::{Application, Clipboard, Command, Container, Element, Length, Subscription, Text, executor, time};

use crate::downloader::{Download, DownloaderConfig, Quality};
use crate::error::SpotifyError;

pub use theme::Theme;
use backend::Backend;
use login_screen::LoginScreen;
use main_screen::MainScreen;
use settings_screen::SettingsScreen;

mod theme;
mod backend;
mod login_screen;
mod main_screen;
mod settings_screen;

/// Start iced UI
pub fn start_ui() -> iced::Result {
    App::run(iced::Settings {
        window: iced::window::Settings {
            size: (600, 700),
            icon: Some(Icon::from_rgba(include_bytes!("../../assets/icon.bin").to_vec(), 64, 64).unwrap()),
            ..Default::default()
        },
        default_font: Some(include_bytes!("../../assets/gotham-book.ttf")),
        ..iced::Settings::default()
    })
}

enum Screens {
    Loading,
    Login {
        screen: LoginScreen
    },
    Main {
        screen: MainScreen
    },
    Settings {
        screen: SettingsScreen
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Empty(()),

    UsernameChanged(String),
    PasswordChanged(String),
    ClientIdChanged(String),
    ClientSecretChanged(String),
    BackendLoad(Result<Backend, SpotifyError>),
    Login,

    QueryChanged(String),
    QuerySubmit, 
    GetDownloadQueue,
    DownloadQueue(Vec<Download>),
    OpenSettings,
    Settings(DownloaderConfig),

    SetDownloadPath(String),
    SetFilenameTemplate(String),
    SetConvertToMP3(bool),
    SetConcurrentDownloads(i32),
    SetQuality(Quality),
    SetSeparator(String),
    SetID3v24(bool),
    DiscardSettings,
    SaveSettings,
    CloseSettings(())
}

struct App {
    theme: Theme,
    screen: Screens,
    backend: Option<Backend>
}

impl App {
    /// Dirty way to get backend from option
    fn backend(&mut self) -> Backend {
        self.backend.as_ref().unwrap().clone()
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    /// Create new instance, connect to backend
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App { 
                theme: Theme::default(),
                screen: Screens::Loading,
                backend: None
            }, 
            Command::perform(Backend::try_load(), Message::BackendLoad)
        )
    }

    fn title(&self) -> String {
        "SpotiDown".to_string()
    }
    
    fn subscription(&self) -> Subscription<Self::Message> {
        match self.backend.is_some() {
            false => Subscription::none(),
            true => time::every(Duration::from_millis(200)).map(|_| Message::GetDownloadQueue)
        }
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::Empty(_) => {}

            // INITIAL LOAD

            Message::BackendLoad(Ok(backend)) => {
                self.backend = Some(backend);
                self.screen = Screens::Main { screen: MainScreen::new(self.theme) };
            }
            Message::BackendLoad(Err(_)) => {
                self.screen = Screens::Login { screen: LoginScreen::new(self.theme) };
            }

            // LOGIN SCREEN MESSAGES

            Message::UsernameChanged(v) => if let Screens::Login { screen, .. } = &mut self.screen {
                screen.username = v;
            },
            Message::PasswordChanged(v) => if let Screens::Login { screen, .. } = &mut self.screen {
                screen.password = v;
            },
            Message::ClientIdChanged(v) => if let Screens::Login { screen, .. } = &mut self.screen {
                screen.client_id = v;
            },
            Message::ClientSecretChanged(v) => if let Screens::Login { screen, .. } = &mut self.screen {
                screen.client_secret = v;
            },
            Message::Login => if let Screens::Login {screen, ..} = &mut self.screen {
                let screen = screen.clone();
                self.screen = Screens::Loading;
                return Command::perform(Backend::login(screen.username, screen.password, screen.client_id, screen.client_secret), Message::BackendLoad);
            }

            // MAIN SCREEN

            Message::QueryChanged(v) => if let Screens::Main { screen } = &mut self.screen {
                screen.query = v;
            }
            Message::QuerySubmit => if let Screens::Main { screen } = &mut self.screen {
                let uri = screen.query.clone();
                return Command::perform(self.backend().add_uri(uri), Message::Empty);
            }
            Message::GetDownloadQueue => if let Screens::Main { .. } = &mut self.screen {
                return Command::perform(self.backend().get_downloads(), Message::DownloadQueue);
            }
            Message::DownloadQueue(q) => if let Screens::Main { screen } = &mut self.screen {
                screen.downloads = q;
            }
            Message::OpenSettings => {
                return Command::perform(self.backend().get_config(), Message::Settings)
            }
            Message::Settings(s) => {
                self.screen = Screens::Settings { screen: SettingsScreen::new(self.theme, s) };
            }


            // SETTINGS

            Message::SetDownloadPath(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.path = Path::new(&v).to_owned();
            }
            Message::SetFilenameTemplate(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.filename_template = v;
            }
            Message::SetConvertToMP3(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.convert_to_mp3 = v;
            }
            Message::SetConcurrentDownloads(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.concurrent_downloads = v as usize;
            }
            Message::SetQuality(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.quality = v;
            }
            Message::SetID3v24(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.id3v24 = v;
            }
            Message::SetSeparator(v) => if let Screens::Settings { screen } = &mut self.screen {
                screen.config.separator = v;
            }
            Message::DiscardSettings => {
                self.screen = Screens::Main { screen: MainScreen::new(self.theme) };
            }
            Message::SaveSettings => if let Screens::Settings { screen } = &mut self.screen {
                let config = screen.config.clone();
                return Command::perform(self.backend().save_config(config), Message::CloseSettings);
            }
            Message::CloseSettings(_) => {
                self.screen = Screens::Main { screen: MainScreen::new(self.theme) };
            }

        }
        Command::none()
    }


    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content: Element<Message> = match &mut self.screen {
            Screens::Loading => Text::new("Loading...").into(),
            Screens::Login { screen} => screen.view(),
            Screens::Main { screen } => screen.view(),
            Screens::Settings { screen } => screen.view(),
        };

        // Full body container
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}