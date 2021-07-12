use iced::{Color, Vector, button, checkbox, container, pick_list, progress_bar, slider, text_input};
use num_traits::Num;

/// iced Theme
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub accent: Color,
    pub active: Color,
    pub hovered: Color,
    pub text: Color,
    pub error: Color,
    pub input_padding: u16,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: "191414".hex_color().unwrap(),
            accent: "1DB954".hex_color().unwrap(),
            active: "1DB954".hex_color().unwrap(),
            hovered: "333333".hex_color().unwrap(),
            text: "FFFFFF".hex_color().unwrap(),
            error: "8D2828".hex_color().unwrap(),
            input_padding: 8,
        }
    }
}

impl container::StyleSheet for Theme {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.background.into(),
            text_color: self.text.into(),
            ..container::Style::default()
        }
    }
}

impl text_input::StyleSheet for Theme {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color { a: 0.3, ..self.hovered }.into(),
            border_radius: 8.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: self.hovered.into(),
            ..self.active()
        }
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            background: Color { a: 0.3, ..self.hovered }.into(),
            ..self.focused()
        }
    }

    fn placeholder_color(&self) -> Color {
        self.hovered
    }

    fn value_color(&self) -> Color {
        self.text
    }

    fn selection_color(&self) -> Color {
        self.accent
    }
}

impl button::StyleSheet for Theme {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: self.accent.into(),
            border_radius: 16.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: self.text,
        }
    }
}

impl progress_bar::StyleSheet for Theme {
    fn style(&self) -> progress_bar::Style {
        progress_bar::Style {
            background: self.hovered.into(),
            bar: self.accent.into(),
            border_radius: 8.0,
        }
    }
}

impl checkbox::StyleSheet for Theme {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color { a: 0.3, ..self.hovered }.into(),
            checkmark_color: self.accent,
            border_radius: 8.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: self.hovered.into(),
            ..self.active(is_checked)
        }
    }
}

impl slider::StyleSheet for Theme {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (self.hovered, self.hovered),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 8.0 },
                color: self.accent,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }
    }

    fn hovered(&self) -> slider::Style {
        slider::Style {
            rail_colors: (self.hovered, self.hovered),
            handle: self.active().handle,
        }
    }

    fn dragging(&self) -> slider::Style {
        slider::Style {
            ..self.active()
        }
    }
}

impl pick_list::StyleSheet for Theme {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            text_color: self.text,
            background: self.hovered.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            selected_text_color: self.accent,
            selected_background: self.hovered.into(),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: self.text,
            background: self.background.into(),
            border_radius: 8.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            icon_size: 1.0,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: self.hovered.into(),
            ..self.active()
        }
    }
}


/// Dark button theme
#[derive(Debug, Clone, Copy)]
pub struct DarkButton {}
impl button::StyleSheet for DarkButton {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: "333333".hex_color().unwrap().into(),
            border_radius: 16.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: "FFFFFF".hex_color().unwrap(),
        }
    }
}

/// Icon button theme
pub struct IconButton {}
impl button::StyleSheet for IconButton {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Color::TRANSPARENT.into(),
            border_radius: 100.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: "FFFFFF".hex_color().unwrap(),
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: "333333".hex_color().unwrap().into(),
            ..self.active()
        }
    }
}


/// Into hex color helper for str
trait IntoHexColor {
    fn hex_color(&self) -> Option<Color>;
}

impl IntoHexColor for str {
    fn hex_color(&self) -> Option<Color> {
        if self.len() == 6 {
            return Some(Color::from_rgb(
                f32::from_str_radix(&self[0..2], 16).ok()? / 255.0,
                f32::from_str_radix(&self[2..4], 16).ok()? / 255.0,
                f32::from_str_radix(&self[4..6], 16).ok()? / 255.0,   
            ));
        }
        if self.len() == 8 {
            return Some(Color::from_rgba(
                f32::from_str_radix(&self[0..2], 16).ok()? / 255.0,
                f32::from_str_radix(&self[2..4], 16).ok()? / 255.0,
                f32::from_str_radix(&self[4..6], 16).ok()? / 255.0,   
                f32::from_str_radix(&self[6..8], 16).ok()? / 255.0,   
            ));
        }
        None
    }
}