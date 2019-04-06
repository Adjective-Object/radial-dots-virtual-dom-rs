use crate::fig::Dot::Dot;
use crate::float_utils::fmax;
use crate::svg::util::translate_svg;
use crate::svg::SvgDrawable::SvgDrawable;
use crate::utf_to_binary::text_to_binary;
use crate::DrawingStyle::DrawingStyle;

pub struct ArcStyle {
    pub radius: f64,
    pub arc_percentage: f64,
    pub arc_offset_percentage: f64,
}

pub struct TextPath<'owner_lifetime> {
    pub zero_dot_style: Option<&'owner_lifetime Dot>,
    pub one_dot_style: Option<&'owner_lifetime Dot>,
    pub arc_style: Option<&'owner_lifetime ArcStyle>,
    pub text: String,
}

impl<'owner_lifetime> TextPath<'owner_lifetime> {
    fn get_arc_style(&self, style: &'owner_lifetime DrawingStyle) -> &'owner_lifetime ArcStyle {
        match self.arc_style {
            Some(style) => style,
            None => &style.default_arc_style,
        }
    }

    fn get_zero_dot_style(&self, style: &'owner_lifetime DrawingStyle) -> &'owner_lifetime Dot {
        match self.zero_dot_style {
            Some(style) => &style,
            None => &style.default_zero_dot_style,
        }
    }

    fn get_one_dot_style(&self, style: &'owner_lifetime DrawingStyle) -> &'owner_lifetime Dot {
        match self.one_dot_style {
            Some(style) => &style,
            None => &style.default_one_dot_style,
        }
    }

    pub fn get_bounding_radius(&self, style: &'owner_lifetime DrawingStyle) -> f64 {
        let arc_style: &'owner_lifetime ArcStyle = self.get_arc_style(style);
        let zero_dot_style: &'owner_lifetime Dot = self.get_zero_dot_style(style);
        let one_dot_style: &'owner_lifetime Dot = self.get_one_dot_style(style);

        return arc_style.radius
            + fmax(
                &one_dot_style.get_bounding_radius(),
                &zero_dot_style.get_bounding_radius(),
            ) / 2.0;
    }
}

impl<'owner_lifetime> SvgDrawable for TextPath<'owner_lifetime> {
    /// Builds an svg for the text path
    ///
    /// The text path is radial and centered on the point (0,0)
    fn as_svg(&self, style: &DrawingStyle) -> String {
        let mut text_binary = match text_to_binary(&self.text) {
            Some(text_binary) => text_binary,
            None => vec![],
        };

        let zero_dot_string: String = self.get_zero_dot_style(style).as_svg(style);
        let one_dot_string: String = self.get_one_dot_style(style).as_svg(style);

        let mut dots: Vec<String> = Vec::with_capacity(text_binary.len());
        let arc_style = self.get_arc_style(style);

        let initial_angle = arc_style.arc_offset_percentage * std::f64::consts::PI * 2.0;
        let arc_range_angle = arc_style.arc_percentage * std::f64::consts::PI * 2.0;
        let num_dots = text_binary.len() - 1;

        for (index, current) in text_binary.iter_mut().enumerate() {
            let dot: &str = if *current {
                &one_dot_string
            } else {
                &zero_dot_string
            };

            let arc_percent = ((index + 1) / (num_dots + 1)) as f64;
            let angle = initial_angle + arc_range_angle * arc_percent;

            let x = arc_style.radius * f64::cos(angle);
            let y = arc_style.radius * f64::sin(angle);

            let moved_dot = translate_svg(&dot, x, y);
            dots.push(moved_dot);
        }

        return dots.join("");
    }
}
