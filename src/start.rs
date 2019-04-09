use crate::drawing_style::DrawingStyle;
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::TextPath;
use crate::svg::svg_drawable::SvgDrawable;
use wasm_bindgen::prelude::*;

// use css_rs_macro::css;
use virtual_dom_rs::*;

struct FullApplicationState {
    style: DrawingStyle,
    diagram: Diagram,
}

fn get_initial_state() -> FullApplicationState {
    return FullApplicationState {
        style: DrawingStyle {
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
        },
        diagram: Diagram {
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
        },
    };
}

trait StatelessComponent<T> {
    fn render(props: &T) -> VirtualNode;
}

struct DotsRenderer {}
impl StatelessComponent<FullApplicationState> for DotsRenderer {
    fn render(props: &FullApplicationState) -> VirtualNode {
        let raw_svg_string: String = props.diagram.as_svg(&props.style);
        // let img_base64_src: String = format!(
        //     "data:image/svg+xml {}",
        //     base64::encode_config(&raw_svg_string, base64::URL_SAFE)
        // );

        return html! {
            {raw_svg_string}
        };
    }
}

struct ColorEditor {}
struct ColorEditorProps<'a> {
    name: &'a str,
    color_id: &'a str,
    value: &'a str,
}
impl<'a> StatelessComponent<ColorEditorProps<'a>> for ColorEditor {
    fn render(props: &ColorEditorProps<'a>) -> VirtualNode {
        return html! {
            <form class="color-editor">
                <label>{props.name}</label>
                <input name={props.color_id} type="color" value={props.value}>
            </form>
        };
    }
}

struct DotEditor {}
impl StatelessComponent<Dot> for DotEditor {
    /// TODO this would be nice to break up into separate forms
    /// as separate components, but there's no concept of Fragments
    /// in this StatelesComponent stuff.
    fn render(props: &Dot) -> VirtualNode {
        return html! {
            <form class="dot-editor">
                <label>Circle Radius</label>
                <input
                    name="circle_radius"
                    type="range"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    value={props.circle_radius} />
                <input
                    name="circle_radius"
                    type="number"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    value={props.circle_radius} />

                <label>Ring Radius</label>
                <input
                    name="ring_radius"
                    type="range"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    value={props.ring_radius} />
                <input
                    name="ring_radius"
                    type="number"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    value={props.circle_radius} />

                <label>Ring Stroke Width</label>
                <input
                    name="ring_stroke_width"
                    type="range"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    value={props.ring_stroke_width} />
                <input
                    name="ring_stroke_width"
                    type="number"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    value={props.circle_radius} />
            </form>
        };
    }
}

struct StyleEditor {}
impl StatelessComponent<DrawingStyle> for StyleEditor {
    fn render(props: &DrawingStyle) -> VirtualNode {
        return html! {
            <div class="style-editor">
                <h3 class="style-editor-subheader">Default One Dot</h3>
                {DotEditor::render(&props.default_one_dot_style)}

                <h3 class="style-editor-subheader">Default Zero Dot</h3>
                {DotEditor::render(&props.default_zero_dot_style)}

                {ColorEditor::render(&ColorEditorProps {
                    name: "Stroke Color",
                    color_id: "background_color",
                    value: &props.stroke_color,
                })}

                {ColorEditor::render(&ColorEditorProps {
                    name: "Background Color",
                    color_id: "background_color",
                    value: &props.background_color,
                })}
            </div>
        };
    }
}

struct RingTextArea {}
impl RingTextArea {
    fn get_as_multiline_text(paths: &Vec<TextPath>) -> String {
        let lines: Vec<String> = paths.iter().map(|path| path.text.clone()).collect();
        return lines.join("\n");
    }
}
impl StatelessComponent<Vec<TextPath>> for RingTextArea {
    fn render(props: &Vec<TextPath>) -> VirtualNode {
        return html! {
            <textarea class="ring-text-area">{RingTextArea::get_as_multiline_text(props)}</textarea>
        };
    }
}

struct Controls {}
impl StatelessComponent<FullApplicationState> for Controls {
    fn render(props: &FullApplicationState) -> VirtualNode {
        return html! {
            <div class="control-bar">
                {RingTextArea::render(&props.diagram.paths)}
                <hr class="controls-divider" />
                {StyleEditor::render(&props.style)}
                <hr class="controls-divider" />
            </div>
        };
    }
}

struct ApplicationRenderer {}
impl StatelessComponent<FullApplicationState> for ApplicationRenderer {
    fn render(props: &FullApplicationState) -> VirtualNode {
        return html! {
            <div class="app-split">
                {DotsRenderer::render(props)}
                {Controls::render(props)}
            </div>
        };
    }
}

#[wasm_bindgen(start)]
#[allow(dead_code)] // rust improperly marks the wasm entrypoint as dead code
pub fn start() {
    // get the root of the application
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let app_host_result: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> =
        document.query_selector("#application-container");
    let host_elem: web_sys::Element =
        match app_host_result.expect("There should be an #application-container") {
            Some(x) => x,
            None => {
                panic!();
            }
        };

    // boot the app
    let state = get_initial_state();

    let virtual_dom = ApplicationRenderer::render(&state);

    // perform initial render to document
    host_elem.set_inner_html(&virtual_dom.to_string());

    // TODO set up update + diffing system loop
}
