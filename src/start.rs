use crate::fig::Diagram::Diagram;
use crate::fig::Dot::Dot;
use crate::fig::TextPath::ArcStyle;
use crate::fig::TextPath::TextPath;
use crate::DrawingStyle::DrawingStyle;
use wasm_bindgen::prelude::*;
use web_sys::Document;

#[wasm_bindgen(start)]
pub fn main() {
    let mut drawing_style = DrawingStyle {
        stroke_color: "#333333".to_string(),
        background_color: "#EEEEEE".to_string(),
        default_zero_dot_style: Dot {
            circle_radius: 1.0,
            ring_radius: 2.0,
            ring_stroke_width: 0.1,
        },
        default_one_dot_style: Dot {
            circle_radius: 0.5,
            ring_radius: 2.0,
            ring_stroke_width: 0.0,
        },
        default_arc_style: ArcStyle {
            radius: 5.0,
            arc_percentage: 1.0,
            arc_offset_percentage: 0.0,
        },
    };

    let diagram = Diagram {
        diagram_padding: 5.0,
        paths: vec![
            TextPath {
                text: "he".to_string(),
                zero_dot_style: None,
                one_dot_style: None,
                arc_style: None,
            },
            TextPath {
                text: "ll".to_string(),
                zero_dot_style: None,
                one_dot_style: None,
                arc_style: None,
            },
            TextPath {
                text: "o".to_string(),
                zero_dot_style: None,
                one_dot_style: None,
                arc_style: None,
            },
        ],
    };
}
