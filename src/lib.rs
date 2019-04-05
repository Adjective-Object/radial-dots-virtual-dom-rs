use wasm_bindgen::prelude::*;

// Datatypes
struct DotStyle {
    circle_radius: f64,
    ring_radius: f64,
    ring_stroke_width: f64,
}

struct ArcStyle {
    radius: f64,
    arc_percentage: f64,
    arc_offset_percentage: f64,
}

struct TextPath<'owner_lifetime> {
    ownerDiagram: &'owner_lifetime Diagram<'owner_lifetime>,
    zero_dot_style: Option<&'owner_lifetime DotStyle>,
    one_dot_style: Option<&'owner_lifetime DotStyle>,
    arc_style: Option<&'owner_lifetime ArcStyle>,
    text: String,
}

struct Diagram<'diagram_lifetime> {
    paths: Vec<TextPath<'diagram_lifetime>>,
    default_zero_dot_style: &'diagram_lifetime DotStyle,
    default_one_dot_style: &'diagram_lifetime DotStyle,
    default_arc_style: &'diagram_lifetime ArcStyle,
}

// library methods

fn getBinary(input: &str) -> Option<Vec<bool>> {
    let mut cur = [0];
    let mut result: Vec<bool> = Vec::with_capacity(input.len() * 8);
    for c in input.chars() {
        if !c.is_ascii() {
            return None;
        }
        c.encode_utf8(&mut cur);

        for i in 0..8 {
            result.push((1 << i) & i != 0);
        }
    }
    return Some(result);
}

fn fmax(a: &f64, b: &f64) -> f64 {
    return if a > b { *a } else { *b };
}

// rendering methods

fn translate_svg(svgString: &str, x: f64, y: f64) -> String {
    format!(
        "<g transform=\"translate({x}, {y})\">{content}</g>",
        x = x,
        y = y,
        content = svgString
    )
}

trait SvgDrawable {
    fn as_svg(&self, stroke_color: &str) -> String;
}

impl DotStyle {
    fn get_bounding_radius(&self) -> f64 {
        let ring_radius: f64 = self.ring_radius + self.ring_stroke_width / 2.0;
        return fmax(&self.circle_radius, &ring_radius);
    }
}

impl SvgDrawable for DotStyle {
    fn as_svg(&self, stroke_color: &str) -> String {
        format!(
            concat!(
                "<circle r={circle_radius} fill={stroke_color} />",
                "<circle r={ring_radius} ",
                "fill=\"transparent\" ",
                "stroke={stroke_color} ",
                "stroke-width={ring_stroke_width} />",
            ),
            circle_radius = self.circle_radius,
            ring_radius = self.ring_radius,
            ring_stroke_width = self.ring_stroke_width,
            stroke_color = stroke_color,
        )
    }
}

impl<'owner_lifetime> TextPath<'owner_lifetime> {
    fn get_arc_style(&self) -> &'owner_lifetime ArcStyle {
        match self.arc_style {
            Some(style) => style,
            None => &self.ownerDiagram.default_arc_style,
        }
    }

    fn get_zero_dot_style(&self) -> &'owner_lifetime DotStyle {
        match self.zero_dot_style {
            Some(style) => &style,
            None => &self.ownerDiagram.default_zero_dot_style,
        }
    }

    fn get_one_dot_style(&self) -> &'owner_lifetime DotStyle {
        match self.one_dot_style {
            Some(style) => &style,
            None => &self.ownerDiagram.default_one_dot_style,
        }
    }

    fn get_bounding_radius(&self) -> f64 {
        let arc_style: &'owner_lifetime ArcStyle = self.get_arc_style();
        let zero_dot_style: &'owner_lifetime DotStyle = self.get_zero_dot_style();
        let one_dot_style: &'owner_lifetime DotStyle = self.get_one_dot_style();

        return arc_style.radius
            + fmax(
                &one_dot_style.get_bounding_radius(),
                &zero_dot_style.get_bounding_radius(),
            ) / 2.0;
    }
}

impl<'owner_lifetime> SvgDrawable for TextPath<'owner_lifetime> {
    fn as_svg(&self, stroke_color: &str) -> String {
        let mut text_binary = match getBinary(&self.text) {
            Some(text_binary) => text_binary,
            None => vec![],
        };

        let zero_dot_string: String = self.get_zero_dot_style().as_svg(stroke_color);
        let one_dot_string: String = self.get_one_dot_style().as_svg(stroke_color);

        let mut dots: Vec<String> = Vec::with_capacity(text_binary.len());
        let arc_style = self.get_arc_style();

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

impl<'owner_lifetime> SvgDrawable for Diagram<'owner_lifetime> {}

impl<'diagram_lifetime> Diagram<'diagram_lifetime> {}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn renderToSvg(name: &str) {
    alert(&format!("Hello, {}!", name));
}
