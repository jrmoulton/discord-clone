mod top_bar;

use crate::ColorPalette::*;
use floem::{
    reactive::create_rw_signal,
    style::{JustifyContent, Style},
    view::View,
    views::*,
    AppContext,
};

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
                    text_input(text_buffer).style(|| {
                        Style::BASE
                            .height_px(50.0)
                            .border_radius(15.0)
                            .background(Dark4.color())
                            .margin_px(15.0)
                            .justify_center()
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
            .flex_grow(2.0)
    })
}
