use floem::peniko;

mod user_icon;
pub use user_icon::*;

mod left_panel;
pub use left_panel::*;

mod middle_panel;
pub use middle_panel::*;

mod icons;
pub use icons::*;

mod main_content;
pub use main_content::main_view;

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum ColorPalette {
    Dark1,
    Dark2,
    Dark3,
    Dark4,
    Light1,
    Light2,
    Light3,
    Light4,
    Green,
    Blue,
}

fn increase_lightness(color: &mut colorsys::Hsl, amount: f64) {
    color.set_lightness(color.lightness() + amount);
}

impl ColorPalette {
    fn mut_color(&self) -> colorsys::Hsl {
        match self {
            ColorPalette::Dark1 => colorsys::Rgb::from((30, 31, 34)).into(),
            ColorPalette::Dark2 => {
                let mut dark1 = Self::Dark1.mut_color();
                increase_lightness(&mut dark1, 5.0);
                dark1
            },
            ColorPalette::Dark3 => {
                let mut dark1 = Self::Dark1.mut_color();
                increase_lightness(&mut dark1, 10.0);
                dark1
            },
            ColorPalette::Dark4 => {
                let mut dark1 = Self::Dark1.mut_color();
                increase_lightness(&mut dark1, 15.0);
                dark1
            },
            ColorPalette::Light1 => colorsys::Rgb::from((242, 243, 245)).into(),
            ColorPalette::Light2 => {
                let mut light1 = Self::Light1.mut_color();
                increase_lightness(&mut light1, -10.0);
                light1
            },
            ColorPalette::Light3 => {
                let mut light1 = Self::Light1.mut_color();
                increase_lightness(&mut light1, -20.0);
                light1
            },
            ColorPalette::Light4 => {
                let mut light1 = Self::Light1.mut_color();
                increase_lightness(&mut light1, -30.0);
                light1
            },
            ColorPalette::Green => colorsys::Rgb::from((80, 164, 97)).into(),
            ColorPalette::Blue => colorsys::Rgb::from((116, 124, 240)).into(),
        }
    }

    pub fn color(&self) -> peniko::Color {
        let color = colorsys::Rgb::from(self.mut_color());

        peniko::Color::rgb(
            color.red() / 255.0,
            color.green() / 255.0,
            color.blue() / 255.0,
        )
    }
}
