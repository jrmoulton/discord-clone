use crate::ColorPalette::*;
use floem::{style::Style, view::View, views::*};

mod servers_list;

pub fn left_panel() -> impl View {
    stack(|| {
        (
            crate::server_icon(
                include_str!("../../discord-icon.svg").to_string(),
                Blue.color(),
            ),
            // Divider
            empty().style(|| {
                Style::BASE
                    .width_px(35.0)
                    .height_px(0.0)
                    .border_color(Dark4.color())
                    .border(1.0)
                    .border_radius(3.0)
            }),
            servers_list::servers_list(),
            crate::server_icon(super::icons::PLUS.to_string(), Green.color()),
            crate::server_icon(super::icons::COMPASS.to_string(), Green.color()),
        )
    })
    .style(|| Style::BASE.flex_col().padding_px(15.0).items_center())
}
