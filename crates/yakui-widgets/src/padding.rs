use yakui_core::paint::PaintDom;
use yakui_core::{dom::Dom, layout::LayoutDom, Constraints, Index, Vec2, Widget};

use crate::util::widget_children;

#[derive(Debug, Clone)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn even(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}

#[derive(Debug)]
pub struct PaddingWidget {
    props: Padding,
}

pub type PadResponse = ();

impl Widget for PaddingWidget {
    type Props = Padding;
    type Response = PadResponse;

    fn new(_index: Index, props: Self::Props) -> Self {
        Self { props }
    }

    fn update(&mut self, props: Self::Props) {
        self.props = props;
    }

    fn layout(&self, dom: &Dom, layout: &mut LayoutDom, input: Constraints) -> Vec2 {
        let node = dom.get_current();

        let mut self_size = Vec2::ZERO;

        let total_padding = Vec2::new(
            self.props.left + self.props.right,
            self.props.top + self.props.bottom,
        );
        let offset = Vec2::new(self.props.left, self.props.top);

        let child_constraints = Constraints {
            min: input.min - total_padding,
            max: input.max - total_padding,
        };

        for &child in &node.children {
            self_size = layout.calculate(dom, child, child_constraints) + total_padding;
            layout.set_pos(child, offset);
        }

        input.constrain(self_size)
    }

    fn paint(&self, dom: &Dom, layout: &LayoutDom, paint: &mut PaintDom) {
        let node = dom.get_current();
        for &child in &node.children {
            paint.paint(dom, layout, child);
        }
    }

    fn respond(&mut self) -> Self::Response {}
}

pub fn pad<F: FnOnce()>(props: Padding, children: F) -> PadResponse {
    widget_children::<PaddingWidget, _>(children, props)
}
