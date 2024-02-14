use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{Document, Element, HtmlElement};
use zkmr_types::Submission;

fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let switch = create_toggle(&document)?;
    body.append_child(&switch)?;

    let submit = create_button(&document, "submit", || {
        // TODO: change depending on toggle
        // TODO: generate real proof
        let submission = Submission {
            job_id: 0,
            result: 0,
            proof: "The proof".into(),
        };
        let json = serde_json::to_string(&submission).unwrap_or_default();
        submit_to_server(json);
    })?;
    body.append_child(&submit)?;

    Ok(())
}

fn create_toggle(document: &Document) -> Result<Element, JsValue> {
    let switch = document.create_element("label")?;
    switch.set_attribute("class", "switch")?;
    let input = document.create_element("input")?;
    input.set_attribute("type", "checkbox")?;
    switch.append_child(&input)?;
    let span = document.create_element("span")?;
    span.set_attribute("class", "slider round")?;
    switch.append_child(&span)?;
    Ok(switch)
}

fn create_button<F: FnMut() + 'static>(
    document: &Document,
    text: &str,
    onclick: F,
) -> Result<HtmlElement, JsValue> {
    let button = document.create_element("button")?;
    button.set_text_content(Some(text));
    let closure = Closure::new::<Box<dyn FnMut()>>(Box::new(onclick));
    let elem: HtmlElement = button.dyn_into().unwrap();
    elem.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    Ok(elem)
}

#[wasm_bindgen(inline_js = r#"export function submit_to_server(x) {
        fetch("/submit", {
            credentials: "same-origin",
            mode: "same-origin",
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: x
        });
    }"#)]
extern "C" {
    fn submit_to_server(x: String);
}
