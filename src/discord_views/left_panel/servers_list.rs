use crate::server_icon;
use floem::{style::Style, view::View, views::*};

use crate::ColorPalette;

pub fn servers_list() -> impl View {
    scroll(|| {
        virtual_list(
            VirtualListDirection::Vertical,
            VirtualListItemSize::Fixed(Box::new(|| 45.0)),
            || im::Vector::from_iter((1..=5).map(|val| val.to_string())),
            |server_data| server_data.clone(),
            move |_name| {
                server_icon(
                    include_str!("../../../discord-icon.svg").to_string(),
                    ColorPalette::Blue.color(),
                )
            },
        )
        .style(|| Style::BASE.flex_col())
    })
    .hide_bar(|| true)
    .style(|| {
        Style::BASE
            // .padding_right_px(15.0)
            .color(ColorPalette::Dark1.color())
    })
}
