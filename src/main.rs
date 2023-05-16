mod discord_views;

use discord_views::*;
use floem::{
    peniko::kurbo::Size,
    reactive::provide_context,
    style::Style,
    view::View,
    views::*,
    window::WindowConfig,
    AppContext,
};

#[derive(Clone)]
struct TopBarWidth(f32);

fn app_logic() -> impl View {
    let cx = AppContext::get_current();
    provide_context(cx.scope, TopBarWidth(50.0));
    stack(|| {
        (
            // left side bar
            left_panel(),
            middle_panel(),
            main_view(),
        )
    })
    .style(|| {
        Style::BASE
            .flex_row()
            // .min_width_px(150.0)
            .size_pct(100.0, 100.0)
    })
}

#[allow(unused)]
fn wrapper<V: View>(child: impl Fn() -> V) -> impl View {
    container(child).style(|| {
        Style::BASE
            .min_width_px(500.0)
            .size_pct(100.0, 100.0)
            .background(ColorPalette::Dark1.color())
    })
}

fn main() {
    // floem::launch(|_cx| app_logic());
    floem::Application::new()
        .window(
            |_cx| app_logic(),
            Some(
                WindowConfig::default()
                    .show_titlebar(true)
                    .size(Size::new(100.0, 100.0)),
            ),
        )
        .run();
}
