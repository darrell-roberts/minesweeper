//! This is a new type for Button. This button widget adds support to handling
//! right mouse click.
use iced::{
    advanced::{self, widget::Tree, Widget},
    event::Status,
    mouse,
    widget::{button::StyleSheet, Button},
    Element, Event, Length, Padding,
};

pub struct CellButton<'a, Message, Renderer = iced::Renderer>
where
    Renderer: advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    inner: Button<'a, Message, Renderer>,
    on_right_click: Option<Message>,
}

impl<'a, Message, Renderer> CellButton<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: advanced::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            inner: Button::new(content),
            on_right_click: None,
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length> + Clone) -> Self {
        self.inner = self.inner.width(width.clone());
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length> + Clone) -> Self {
        self.inner = self.inner.height(height.clone());
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.inner = self.inner.on_press(on_press);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.inner = self.inner.on_press_maybe(on_press);
        self
    }

    pub fn on_right_press(mut self, on_right_press: Message) -> Self {
        self.on_right_click = Some(on_right_press);
        self
    }

    /// Sets the style variant of this [`Button`].
    pub fn style(
        mut self,
        style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self {
        self.inner = self.inner.style(style);
        self
    }

    fn as_widget(&self) -> &dyn Widget<Message, Renderer> {
        &self.inner as &dyn Widget<Message, Renderer>
    }

    fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Renderer> {
        &mut self.inner as &mut dyn Widget<Message, Renderer>
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for CellButton<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> advanced::widget::tree::Tag {
        self.as_widget().tag()
    }
    fn state(&self) -> advanced::widget::tree::State {
        self.as_widget().state()
    }
    fn children(&self) -> Vec<advanced::widget::Tree> {
        self.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.as_widget().diff(tree)
    }

    fn width(&self) -> Length {
        self.as_widget().width()
    }

    fn height(&self) -> Length {
        self.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced::advanced::Renderer>::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.as_widget()
            .draw(state, renderer, theme, style, layout, cursor, viewport)
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn advanced::widget::Operation<Message>,
    ) {
        self.as_widget().operate(state, layout, renderer, operation)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced::Event,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn advanced::Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::event::Status {
        let status = self.as_widget_mut().on_event(
            state,
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if status == Status::Ignored {
            // Try right mouse.
            if let Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Right,
            )) = event
            {
                if let Some(on_right_press) = self.on_right_click.clone() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        shell.publish(on_right_press);
                    }

                    return Status::Captured;
                }
            }
        }

        status
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> advanced::mouse::Interaction {
        self.as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: advanced::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<advanced::overlay::Element<'b, Message, Renderer>> {
        self.as_widget_mut().overlay(state, layout, renderer)
    }
}

pub fn cell_button<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> CellButton<'a, Message, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Renderer::Theme: StyleSheet,
    <Renderer::Theme as StyleSheet>::Style: Default,
    Message: Clone + 'a,
{
    CellButton::new(content)
}

impl<'a, Message, Renderer> From<CellButton<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: advanced::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(value: CellButton<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}
