#![allow(missing_docs)]

use gpui::Axis;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct ToolStrip {
    id: ElementId,
    tools: Vec<IconButton>,
    axis: Axis,
}

impl ToolStrip {
    fn new(id: ElementId, axis: Axis) -> Self {
        Self {
            id,
            tools: vec![],
            axis,
        }
    }

    pub fn vertical(id: impl Into<ElementId>) -> Self {
        Self::new(id.into(), Axis::Vertical)
    }

    pub fn tools(mut self, tools: Vec<IconButton>) -> Self {
        self.tools = tools;
        self
    }

    pub fn tool(mut self, tool: IconButton) -> Self {
        self.tools.push(tool);
        self
    }
}

impl RenderOnce for ToolStrip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let group = format!("tool_strip_{}", self.id.clone());

        div()
            .id(self.id.clone())
            .group(group)
            .map(|element| match self.axis {
                Axis::Vertical => element.v_flex(),
                Axis::Horizontal => element.h_flex(),
            })
            .flex_none()
            .gap(DynamicSpacing::Base04.rems(cx))
            .p(DynamicSpacing::Base02.rems(cx))
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded(rems_from_px(6.0))
            .bg(cx.theme().colors().elevated_surface_background)
            .children(self.tools)
    }
}
