use crate::drawing_style::DrawingStyle;
use crate::fig::diagram::Diagram;
use crate::fig::dot::Dot;
use crate::fig::text_path::ArcStyle;
use crate::fig::text_path::TextPath;
use crate::svg::svg_drawable::SvgDrawable;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;

// use css_rs_macro::css;
use virtual_dom_rs::*;

#[derive(Debug)]
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
        let img_base64_src: String = format!(
            "data:image/svg+xml;base64,{}",
            base64::encode_config(&raw_svg_string, base64::STANDARD)
        );

        return html! {
            <img class="dot-ring-img" src=img_base64_src />
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
struct DotProps<'a> {
    dot: &'a Dot,
    dot_id: &'a str,
}

impl<'a> StatelessComponent<DotProps<'a>> for DotEditor {
    /// TODO this would be nice to break up into separate forms
    /// as separate components, but there's no concept of Fragments
    /// in this StatelesComponent stuff.
    fn render(props: &DotProps) -> VirtualNode {
        return html! {
            <form class="dot-editor">
                <label>Circle Radius</label>
                <input
                    name="circle_radius"
                    type="range"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    data_input_type="dot_input"
                    data_dot_id={props.dot_id}
                    value={props.dot.circle_radius} />
                <input
                    name="circle_radius"
                    type="number"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    data_input_type="dot_input"
                    data_dot_id={props.dot_id}
                    value={props.dot.circle_radius} />

                <label>Ring Radius</label>
                <input
                    name="ring_radius"
                    type="range"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    data_input_type="dot_input"
                    data_dot_id={props.dot_id}
                    value={props.dot.ring_radius} />
                <input
                    name="ring_radius"
                    type="number"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    data_input_type="dot_input"
                    data_dot_id={props.dot_id}
                    value={props.dot.circle_radius} />

                <label>Ring Stroke Width</label>
                <input
                    name="ring_stroke_width"
                    type="range"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    data_input_type="dot_input"
                    data_dot_id={props.dot_id}
                    value={props.dot.ring_stroke_width} />
                <input
                    name="ring_stroke_width"
                    type="number"
                    step="0.1"
                    min="0.0"
                    max="10.0"
                    data_input_type="dot_input"
                    data_dot_id={props.dot_id}
                    value={props.dot.circle_radius} />
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
                {DotEditor::render(&DotProps {
                    dot: &props.default_one_dot_style,
                    dot_id: "default-one-dot"
                })}

                <h3 class="style-editor-subheader">Default Zero Dot</h3>
                {DotEditor::render(&DotProps {
                    dot: &props.default_zero_dot_style,
                    dot_id: "default-zero-dot"
                })}

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
        let background_style = format!("background-color: {}", props.style.background_color);
        return html! {
            <div class="app-split" style=background_style>
                {DotsRenderer::render(props)}
                {Controls::render(props)}
            </div>
        };
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct AppMount {
    dom: VirtualNode,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct App {
    state: FullApplicationState,
    mount_state: Option<AppMount>,
}

/// Create a new app and return it
#[wasm_bindgen]
#[allow(dead_code)]
pub fn init_app() -> App {
    return App {
        state: get_initial_state(),
        mount_state: None,
    };
}

/// Mounts the app at the given seelctor.
///
/// Returns true iff the app mounted incorrectly
/// Will throw if no element with an appropriate selector is found
/// or if the provided app already has a dom.
#[wasm_bindgen]
#[allow(dead_code)]
pub fn mount(host_elem: Element, app: &mut App) -> bool {
    match &app.mount_state {
        Some(_x) => {
            return false;
        }
        None => {
            let next_dom = ApplicationRenderer::render(&app.state);
            host_elem.set_inner_html(&next_dom.to_string());
            app.mount_state = Some(AppMount { dom: next_dom });
            return true;
        }
    };
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn action_update_default_dot(
    app: &mut App,
    dot_id: &str,
    input_name: &str,
    new_value: f64,
) -> bool {
    web_sys::console::log_1(
        &format!(
            "rust called with {:?} {:?} {:?}",
            &dot_id, input_name, new_value
        )
        .into(),
    );

    let default_dot: &mut Dot = match dot_id.as_ref() {
        "default-one-dot" => &mut app.state.style.default_one_dot_style,
        "default-zero-dot" => &mut app.state.style.default_zero_dot_style,
        _ => {
            web_sys::console::log_1(&format!("failed to match dot_id {:?}", &dot_id).into());
            return false;
        }
    };

    web_sys::console::log_1(&format!("bound dot",).into());

    match input_name.as_ref() {
        "circle_radius" => {
            web_sys::console::log_1(&format!("write circle_radius",).into());
            default_dot.circle_radius = new_value;
        }
        "ring_radius" => {
            web_sys::console::log_1(&format!("write ring_radius",).into());
            default_dot.ring_radius = new_value;
        }
        "ring_stroke_width" => {
            web_sys::console::log_1(&format!("write ring_stroke_width",).into());
            default_dot.ring_stroke_width = new_value;
        }
        _ => {
            web_sys::console::log_1(&format!("unexpected input name {:?}", &input_name).into());
            return false;
        }
    };

    return true;
}

/// Updates the rendered app
#[allow(dead_code)]
#[wasm_bindgen]
pub fn rerender_app(host_elem: Element, app: &mut App) -> bool {
    web_sys::console::log_1(&"rendering app".into());
    match &mut app.mount_state {
        None => {
            web_sys::console::log_1(&"no mount state - not rerendering".into());
            return false;
        }
        Some(mount_state) => {
            web_sys::console::log_1(&"rendering".into());
            let next_dom = ApplicationRenderer::render(&app.state);
            web_sys::console::log_1(&"diffing".into());
            let patches = virtual_dom_rs::diff(&mount_state.dom, &next_dom);

            // dom patching consumes the node
            web_sys::console::log_1(&format!("applying patches {:?}", patches).into());
            let patch_result = virtual_dom_rs::patch(host_elem, &patches);

            match patch_result {
                Ok(_) => {}
                Err(e) => {
                    web_sys::console::log_1(&format!("failure during patching {:?}", e).into())
                }
            }

            web_sys::console::log_1(&"updating mount state".into());
            app.mount_state = Some(AppMount { dom: next_dom });
        }
    };

    return true;
}
