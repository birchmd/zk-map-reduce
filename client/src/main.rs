use {
    wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue},
    web_sys::{
        Document, Element, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlSelectElement,
    },
    zkmr_types::Submission,
};

mod prover;

const PROGRAMS_ID: &str = "programs";
const WORKER_ID: &str = "worker";
const DROPDOWN_ID: &str = "job_ids";
const SUBMIT_ID: &str = "submit";

fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let miden_programs = prover::compile_programs();

    // Short paragraph explaining what is going on
    let p = document.create_element("p")?;
    p.set_text_content(Some("You are a worker in our map-reduce! You are supposed to run the squaring program, but will you?"));
    body.append_child(&p)?;
    body.append_child(document.create_element("br").as_ref()?)?;

    // Worker ID text input
    let worker_id_label = create_label_for(&document, WORKER_ID, "Worker ID: ")?;
    body.append_child(&worker_id_label)?;
    let woker_id = document.create_element("input")?;
    woker_id.set_attribute("id", WORKER_ID)?;
    woker_id.set_attribute("type", "text")?;
    body.append_child(&woker_id)?;
    let worker_id: HtmlInputElement = woker_id.dyn_into().unwrap();
    body.append_child(document.create_element("br").as_ref()?)?;
    body.append_child(document.create_element("br").as_ref()?)?;

    // Program selection dropdown
    let programs_label = create_label_for(&document, PROGRAMS_ID, "Program to run: ")?;
    body.append_child(&programs_label)?;
    let programs_dd = create_dropdown(&document, DROPDOWN_ID, prover::PROGRAM_NAMES)?;
    body.append_child(&programs_dd)?;
    body.append_child(document.create_element("br").as_ref()?)?;
    body.append_child(document.create_element("br").as_ref()?)?;

    // Job ID dropdown selection
    let jobs_label = create_label_for(&document, DROPDOWN_ID, "Job ID: ")?;
    body.append_child(&jobs_label)?;
    let dropdown = create_dropdown(&document, DROPDOWN_ID, (1..=10).map(|i| i.to_string()))?;
    body.append_child(&dropdown)?;
    body.append_child(document.create_element("br").as_ref()?)?;
    body.append_child(document.create_element("br").as_ref()?)?;

    // Submit button
    let submit = create_button(&document, "Submit Proof", SUBMIT_ID, move |button| {
        button.set_disabled(true); // Disable button while proof is being generated
        let job_id: u8 = dropdown.value().parse().unwrap();
        let program_name = programs_dd.value();
        let miden_program = miden_programs
            .get(program_name.as_str())
            .expect("Program must be in the map");
        let proof = prover::prove(miden_program, job_id);
        let result = proof.output_stack.first().copied().unwrap();
        let overflow_addrs = proof.overflow_addrs.iter().map(|x| x.to_string()).collect();
        let encoded_proof = proof.encoded();
        let submission = Submission {
            worker_id: worker_id.value(),
            job_id,
            output_stack: proof.output_stack,
            overflow_addrs,
            proof: encoded_proof,
        };
        let json = serde_json::to_string(&submission).unwrap_or_default();
        let msg = format!("Proof submitted for job ID {job_id} with result {result}");
        submit_to_server(json, msg);
        button.set_disabled(false);
    })?;
    body.append_child(&submit)?;

    Ok(())
}

fn create_label_for(document: &Document, id: &str, text: &str) -> Result<Element, JsValue> {
    let label = document.create_element("label")?;
    label.set_text_content(Some(text));
    label.set_attribute("for", id)?;
    Ok(label)
}

fn create_dropdown<I, S>(
    document: &Document,
    id: &str,
    options: I,
) -> Result<HtmlSelectElement, JsValue>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let dd = document.create_element("select")?;
    dd.set_attribute("id", id)?;
    dd.set_attribute("name", id)?;

    for option in options {
        let el = document.create_element("option")?;
        el.set_attribute("value", option.as_ref())?;
        el.set_text_content(Some(option.as_ref()));
        dd.append_child(&el)?;
    }

    Ok(dd.dyn_into().unwrap())
}

fn create_button<F: FnMut(HtmlButtonElement) + 'static>(
    document: &Document,
    id: &str,
    text: &str,
    mut onclick: F,
) -> Result<HtmlButtonElement, JsValue> {
    let button = document.create_element("button")?;
    button.set_attribute("id", id)?;
    button.set_text_content(Some(text));
    let self_ref: HtmlButtonElement = button.clone().dyn_into().unwrap();
    let closure = Closure::new::<Box<dyn FnMut()>>(Box::new(move || onclick(self_ref.clone())));
    let elem: HtmlElement = button.dyn_into().unwrap();
    elem.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    Ok(elem.dyn_into().unwrap())
}

#[wasm_bindgen(inline_js = r#"export function submit_to_server(x, alert_msg) {
        fetch("/submit", {
            credentials: "same-origin",
            mode: "same-origin",
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: x
        });
        alert(alert_msg);
    }"#)]
extern "C" {
    fn submit_to_server(x: String, alert_msg: String);
}
