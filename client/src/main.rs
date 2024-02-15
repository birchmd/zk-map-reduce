use {
    self::toggle::Toggle,
    wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue},
    web_sys::{Document, HtmlElement},
    zkmr_types::Submission,
};

mod toggle;

fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let toggle = Toggle::new(&document)?;
    toggle.append_to_body(&body)?;
    body.append_child(document.create_element("br").as_ref()?)?;

    let submit = create_button(&document, "submit", move || {
        // TODO: generate real proof
        let proof = if toggle.is_checked() {
            "A dishonest proof"
        } else {
            "The honest proof"
        };
        let submission = Submission {
            job_id: 0,
            result: 0,
            proof: proof.into(),
        };
        let json = serde_json::to_string(&submission).unwrap_or_default();
        submit_to_server(json);
    })?;
    body.append_child(&submit)?;

    Ok(())
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
