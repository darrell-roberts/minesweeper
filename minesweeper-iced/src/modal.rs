use iced::{
    padding,
    widget::{container, mouse_area, opaque, stack},
    Color, Element, Length,
};

pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(
                container(opaque(content))
                    .align_top(Length::Fill)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(padding::top(10))
                    .style(|_theme| {
                        container::Style {
                            background: Some(
                                Color {
                                    a: 0.8,
                                    ..Color::BLACK
                                }
                                .into(),
                            ),
                            ..container::Style::default()
                        }
                    })
            )
            .on_press(on_blur)
        )
    ]
    .into()
}
