use iced::{
    advanced::{
        self,
        layout::Node,
        renderer::{self, Quad},
        Widget,
    },
    color,
    widget::{
        self,
        text::{Catalog, State},
    },
    Background, Border, Length, Size,
};

type Message = crate::Message;

pub struct Tag<'a> {
    text: iced::widget::Text<'a>,
    width: Length,
    height: Length,
}

impl<'a> Tag<'a> {
    pub fn new(content: &'a str) -> Self {
        let text = widget::text(content);
        Self {
            text,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Tag<'_>
where
    Renderer: 'a + renderer::Renderer + advanced::text::Renderer,
    Theme: Catalog,
{
    fn layout(
        &self,
        _tree: &mut advanced::widget::Tree,
        _renderer: &Renderer,
        _limits: &advanced::layout::Limits,
    ) -> advanced::layout::Node {
        Node::default()
    }

    fn draw(
        &self,
        tree: &advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
        let mut col = color!(0, 0, 0);
        let pos = layout.position();
        if cursor.is_over(layout.bounds()) {
            col = color!(10, 0, 10);
        }
        let quad = Quad {
            border: Border::default().width(3).rounded(5),
            ..Default::default()
        };
        renderer.fill_quad(quad, Background::Color(col));
        renderer.fill_paragraph(state.0.raw(), pos, color!(255, 255, 255), layout.bounds());
    }

    fn size(&self) -> Size<iced::Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}
