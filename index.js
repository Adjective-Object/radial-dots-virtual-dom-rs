// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import("./pkg/radial_dots");

rust
  .then(dotAppModule => {
    const app = dotAppModule.init_app();
    const host = document.getElementById("application-container");
    dotAppModule.mount(host, app);

    // Add default dot event handler
    document.addEventListener("input", e => {
      const target = e.target;
      const dot_id = target.getAttribute("data_dot_id");
      const input_name = target.getAttribute("name");
      const new_value = parseFloat(target.value);
      if (dot_id.startsWith("default")) {
        const updated = dotAppModule.action_update_default_dot(
          app,
          dot_id,
          input_name,
          new_value
        );
        if (updated) {
          dotAppModule.rerender_app(host, app);
        }
      }
    });
  })
  .catch(console.error);
