use crate::drawing_style::DrawingStyle;
use crate::fig::text_path::*;
use crate::float_utils::fmax;
use crate::geom::*;
use crate::svg::svg_drawable::SvgDrawable;
use crate::svg::util::*;

pub struct Diagram {
    pub paths: Vec<TextPath>,
    pub diagram_padding: f64,
}

impl Diagram {
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

impl SvgDrawable for Diagram {
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

        return format!(
            "<svg viewBox='{} {} {} {}'>{}</svg>",
            diagram_bounds.x,
            diagram_bounds.y,
            diagram_bounds.width,
            diagram_bounds.height,
            svg_parts.join("")
        );
    }
}
