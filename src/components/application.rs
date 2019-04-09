#![feature(proc_macro_hygiene)]

extern crate css_rs_macro;
use css_rs_macro::css;
use virtual_dom_rs::prelude::*;


fn () -> VirtualNode {
    let another_component_css = css! {r#"
    :host {
        display: flex;
        flex-direction: column;
    }
    :host > h3 {
        color: blue;
    }
    .red {
        color: red;
    }
    "#};

    let another_component_css = &format!("{} more classes can go here", another_component_css);

    let some_component = html! {
    <h1 class=SOME_COMPONENT_CSS>
        And there we have it
    </h1>
    };

    let another_component = html! {
    <div class=another_component_css>
        <h3> we have some </h3>
        <span class="red"> CSS </span>
    </div>
    };

    html! {
      <div>
       {some_component}
       {another_component}
       <link rel="stylesheet" type="text/css" href="/app.css" />
     </div>
    }
}