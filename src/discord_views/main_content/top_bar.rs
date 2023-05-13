use crate::ColorPalette::*;
use floem::{
    reactive::use_context,
    style::{JustifyContent, Style},
    view::View,
    views::*,
    AppContext,
};

pub fn top_bar() -> impl View {
    // Content and separator
    let top_bar_width = use_context::<crate::TopBarWidth>(AppContext::get_current().scope)
        .unwrap_or(crate::TopBarWidth(30.0))
        .0;
    stack(move || {
        (
            // All the text in the bar
            stack(move || {
                (
                    label(|| "Floem".to_string()),
                    empty().style(move || {
                        Style::BASE
                            .width_px(0.0)
                            .height_px(top_bar_width * (2.0 / 3.0))
                            .border(0.5)
                            .border_radius(3.0)
                            .border_color(Dark4.color())
                    }),
                    label(|| "Channel to discuss floem".to_string()),
                )
            })
            .style(move || {
                Style::BASE
                    .background(Dark3.color())
                    .items_center()
                    .padding_horiz_px(10.0)
                    .height_px(top_bar_width)
                    .flex_row()
                    .justify_content(Some(JustifyContent::SpaceBetween))
            }),
            empty().style(|| {
                Style::BASE
                    .height_px(0.0)
                    .border(0.5)
                    .border_radius(3.0)
                    .border_color(Dark1.color())
            }),
        )
    })
    .style(|| Style::BASE.flex_col())
}
