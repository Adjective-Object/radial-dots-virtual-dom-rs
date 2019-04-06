use crate::fig::Dot::Dot;
use crate::fig::TextPath::ArcStyle;

pub struct DrawingStyle {
    pub stroke_color: String,
    pub background_color: String,
    pub default_zero_dot_style: Dot,
    pub default_one_dot_style: Dot,
    pub default_arc_style: ArcStyle,
}
