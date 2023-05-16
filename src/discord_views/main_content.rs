mod top_bar;

use floem::{
    reactive::create_rw_signal,
    style::{JustifyContent, Style},
    view::View,
    views::*,
    AppContext,
};

use super::icon_plus;
use crate::ColorPalette::*;

pub fn main_view() -> impl View {
    let cx = AppContext::get_current();
    let text_buffer = create_rw_signal(cx.scope, String::new());
    stack(|| {
        (
            top_bar::top_bar(),
            // messages and text input
            stack(|| {
                (
                    scroll(|| {
                        virtual_list(
                            VirtualListDirection::Vertical,
                            VirtualListItemSize::Fn(Box::new(|_message| 15.0)),
                            || {
                                im::Vector::from_iter(
                                    "this is super fancy and stuff and I'm going to write a very long message so that every one knows".split(' ').map(String::from),
                                )
                            },
                            |message| message.clone(),
                            |message| {
                                label(move || message.clone())
                                    .style(|| Style::BASE.padding_horiz_px(15.0).color(Light1.color()))
                            },
                        )
                        .style(|| Style::BASE.flex_col())
                    }),
                    // Text box with elements
                    stack(|| (
                        container(|| icon_plus().style(|| Style::BASE.color(Dark4.color()).size_px(12.0, 12.0))).style(|| Style::BASE.padding_px(5.0).circle(true).background(Light2.color())).hover_style(|| Style::BASE.padding_px(5.0).circle(true).background(Light1.color())),
                        text_input(text_buffer).style(|| Style::BASE.padding_horiz_px(10.0).font_size(17.0).cursor_color(Light2.color()))
                        )
                    ).style(|| {
                        Style::BASE
                            .height_px(50.0)
                            .border_radius(15.0)
                            .background(Dark4.color())
                            .margin_horiz_px(15.0)
                            .margin_vert_px(30.0)
                            .padding_horiz_px(10.0)
                            .color(Light2.color())
                            .items_center()
                    }),
                )
            })
            .style(|| Style::BASE.flex_col()),
        )
    })
    .style(|| {
        Style::BASE
            .background(Dark3.color())
            .flex_col()
            .justify_content(Some(JustifyContent::SpaceBetween))
            // .size_pct(100.0, 100.0)
            .flex_grow(1.0)
    })
}
