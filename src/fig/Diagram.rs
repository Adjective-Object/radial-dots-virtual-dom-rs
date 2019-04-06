use crate::fig::TextPath::*;
use crate::float_utils::fmax;
use crate::geom::*;
use crate::svg::util::*;
use crate::svg::SvgDrawable::SvgDrawable;
use crate::DrawingStyle::DrawingStyle;

pub struct Diagram<'diagram_lifetime> {
    pub paths: Vec<TextPath<'diagram_lifetime>>,
    pub diagram_padding: f64,
}

impl<'diagram_lifetime> Diagram<'diagram_lifetime> {
    fn get_bounding_rect(&self, style: &DrawingStyle) -> Rect {
        let mut radius: f64 = 1.0;
        for path in self.paths.iter() {
            radius = fmax(&radius, &path.get_bounding_radius(style));
        }

        return Rect {
            x: 0.0,
            y: 0.0,
            width: radius + 2.0 * self.diagram_padding,
            height: radius + 2.0 * self.diagram_padding,
        };
    }
}

impl<'diagram_lifetime> SvgDrawable for Diagram<'diagram_lifetime> {
    fn as_svg(&self, style: &DrawingStyle) -> String {
        let mut svg_parts: Vec<String> = Vec::with_capacity(self.paths.len() + 1);
        let diagram_bounds: Rect = self.get_bounding_rect(style);
        let diagram_center: Vector2 = diagram_bounds.center();
        svg_parts.push(rect_svg(&diagram_bounds, &style.background_color));

        for path in self.paths.iter() {
            svg_parts.push(translate_svg(
                &path.as_svg(style),
                diagram_center.x,
                diagram_center.y,
            ));
        }

        return svg_parts.join("");
    }
}
