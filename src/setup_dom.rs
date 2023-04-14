use leptos::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

/**
# Examples

```
use leptos::view;
let test_wrapper = setup_dom(|cx| view! { cx, <div> <p> "hello" </p> </div> });
```
*/
pub fn setup_dom<F, N>(view: F) -> web_sys::Element
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    let document = leptos::document();
    let test_wrapper = document.create_element("section").unwrap();
    document
        .body()
        .unwrap()
        .append_child(&test_wrapper)
        .expect("can't append the test_wrapper into the body");

    // start by rendering our counter and mounting it to the DOM
    // note that we start at the initial value of 10
    mount_to(
        test_wrapper.clone().unchecked_into(),
        view,
    );

    return test_wrapper;
}

