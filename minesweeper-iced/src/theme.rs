use iced::{
    widget::{button, container},
    Background, Color, Theme, Vector,
};

pub struct CellButtonStyle;

impl button::StyleSheet for CellButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 0.5, y: 0.2 },
            background: Some(Background::Color(Color {
                // r: 128. / 255.,
                // g: 0.,
                // b: 64. / 255.,
                r: 102. / 255.,
                g: 153. / 255.,
                b: 0.,
                a: 1.,
            })),
            border_radius: 0.0.into(),
            border_width: 0.4,
            border_color: Color::BLACK,
            // border_color: Color {
            //     r: 230. / 255.,
            //     g: 0.,
            //     b: 115. / 255.,
            //     a: 1.,
            // },
            text_color: Color::BLACK,
        }
    }
}

pub struct ModalStyle;

impl container::StyleSheet for ModalStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color {
                r: 0.,
                g: 153. / 255.,
                b: 204. / 255.,
                a: 0.7,
            })),
            border_radius: 15.0.into(),
            ..Default::default()
        }
    }
}
