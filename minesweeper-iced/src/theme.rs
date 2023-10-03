use iced::{widget::button, Background, Color, Theme, Vector};

pub struct CellButtonStyle;

impl button::StyleSheet for CellButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 0.5, y: 0.2 },
            background: Some(Background::Color(Color {
                r: 128. / 255.,
                g: 0.,
                b: 64. / 255.,
                a: 1.,
            })),
            border_radius: 2.0.into(),
            border_width: 0.,
            border_color: Color::WHITE,
            text_color: Color::BLACK,
        }
    }
}
