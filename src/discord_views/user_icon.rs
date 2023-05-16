use floem::{
    event::EventListner,
    peniko::Color,
    reactive::{create_rw_signal, SignalGet, SignalUpdate},
    style::{Position, Style},
    view::View,
    views::*,
    AppContext,
};

use super::ColorPalette::*;

pub fn server_icon(svg_str: String, color: Color) -> impl View {
    let is_hovered = create_rw_signal(AppContext::get_current().scope, false);
    stack(|| {
        (
            container(move || {
                container(|| {
            svg(move || svg_str.clone())
                .on_click(|_| true)
                .style(move || {
                    Style::BASE
                        .size_px(30.0, 30.0)
                        .color(color)
                        .apply_if(is_hovered.get(), |_| {
                            Style::BASE.size_px(30.0, 30.0).color(Light2.color())
                        })
                })
        })
        .style(move || {
            Style::BASE
                .padding_px(10.0)
                .circle(true)
                .background(Dark3.color())
                .margin_vert_px(5.0)
                .apply_if(is_hovered.get(), |_| {
                    Style::BASE
                        .circle(false)
                        .padding_px(10.0)
                        .margin_vert_px(5.0)
                        .border_radius(20.0)
                        .background(color)
                })
        })
        .on_click(|_| true)
        // This shoudl just be an optional style if the particular server is selected
        .focus_style(|| {
            Style::BASE
                .padding_px(10.0)
                .padding_px(10.0)
                .margin_vert_px(5.0)
                .border_radius(20.0)
                .background(Dark3.color())
        })
        .on_event(EventListner::PointerEnter, move |_| {
            is_hovered.update(|val| *val = true);
            true
        })
        .on_event(EventListner::PointerLeave, move |_| {
            is_hovered.update(|val| *val = false);
            true
        })
            }),
            // label(|| "Serenity.rs".to_string()).style(|| {
            //     Style::BASE
            //         .position(Position::Absolute)
            //         .color(Light1.color())
            //     // .left_px(10.0)
            // }),
        )
    })
}
