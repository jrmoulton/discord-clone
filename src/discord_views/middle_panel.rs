use super::{icons::*, ColorPalette};
use floem::{
    reactive::use_context,
    style::{JustifyContent, Style},
    view::View,
    views::*,
    AppContext,
};
use ColorPalette::*;

pub fn middle_panel() -> impl View {
    stack(|| {
        (
            // Server title and drop_down
            stack(|| {
                (
                    label(|| "Lapce".to_string())
                        .style(|| Style::BASE.color(ColorPalette::Light1.color())),
                    icon_chevron_down()
                        .style(|| Style::BASE.color(Light1.color()).size_px(15.0, 15.0)),
                )
            })
            .style(|| {
                let width = use_context::<crate::TopBarWidth>(AppContext::get_current().scope)
                    .unwrap_or(crate::TopBarWidth(30.0));
                Style::BASE
                    .justify_content(Some(JustifyContent::SpaceBetween))
                    .height_px(width.0)
                    .padding_px(5.0)
            }),
            // separator
            empty().style(|| {
                Style::BASE
                    .height_px(0.0)
                    .border(0.5)
                    .border_radius(3.0)
                    .border_color(Dark1.color())
            }),
            // Channel Browser
            scroll(|| {
                virtual_list(
                    VirtualListDirection::Vertical,
                    VirtualListItemSize::Fixed(Box::new(|| 50.0)),
                    || {
                        im::Vector::from_iter(
                            "roles\nrules\nnew\nfancy".split('\n').map(String::from),
                        )
                    },
                    |channel_name| channel_name.clone(),
                    |channel_name| label(move || format!("N: {channel_name}")),
                )
                .style(|| Style::BASE.flex_col())
            }),
        )
    })
    .style(|| {
        Style::BASE
            .flex_col()
            .width_px(100.0)
            .background(Dark2.color())
    })
}
