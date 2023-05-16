use floem::{
    cosmic_text::Weight,
    reactive::use_context,
    style::{JustifyContent, Style},
    view::View,
    views::*,
    AppContext,
};

use crate::{discord_views::icons::*, ColorPalette::*};

pub fn top_bar() -> impl View {
    // Content and separator
    let top_bar_width = use_context::<crate::TopBarWidth>(AppContext::get_current().scope)
        .unwrap_or(crate::TopBarWidth(30.0))
        .0;
    title_bar(|| {
        stack(move || {
            (
                // All the text in the bar
                stack(move || {
                    (
                        stack(|| {
                            (
                                icon_hashtag().style(|| {
                                    Style::BASE
                                        .size_px(25.0, 25.0)
                                        .color(Light4.color())
                                        .padding_horiz_px(5.0)
                                }),
                                label(|| "Floem".to_string()).style(|| {
                                    Style::BASE
                                        .font_size(17.0)
                                        .font_weight(Weight(500))
                                        .padding_horiz_px(5.0)
                                }),
                            )
                        })
                        .style(|| Style::BASE.items_center().justify_center()),
                        empty().style(move || {
                            Style::BASE
                                .width_px(0.0)
                                .height_px(top_bar_width * (2.0 / 3.0))
                                .border(0.5)
                                .border_radius(3.0)
                                .border_color(Dark4.color())
                        }),
                        label(|| "Channel to discuss floem".to_string())
                            .style(|| Style::BASE.font_size(14.0).color(Light3.color())),
                    )
                })
                .style(move || {
                    Style::BASE
                        .background(Dark3.color())
                        .color(Light1.color())
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
        .style(|| Style::BASE.flex_col().flex_grow(1.0))
    })
}
