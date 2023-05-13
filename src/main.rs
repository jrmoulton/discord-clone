mod discord_views;

use discord_views::*;
use floem::{reactive::provide_context, style::Style, view::View, views::*, AppContext};

fn app_logic() -> impl View {
    stack(|| {
        (
            // left side bar
            left_panel(),
            middle_panel(),
            main_view(),
        )
    })
    .style(|| Style::BASE.flex_row())
}

#[derive(Clone)]
struct TopBarWidth(f32);

fn wrapper<V: View>(child: impl Fn() -> V) -> impl View {
    let cx = AppContext::get_current();
    provide_context(cx.scope, TopBarWidth(30.0));
    container(child).style(|| {
        Style::BASE
            .min_width_px(500.0)
            .size_pct(100.0, 100.0)
            .background(ColorPalette::Dark1.color())
    })
}

fn main() {
    floem::launch(|_cx| wrapper(app_logic));
}
