use {
    wasm_bindgen::{JsCast, JsValue},
    web_sys::{Document, Element, HtmlElement, HtmlInputElement},
};

pub struct Toggle {
    switch: Element,
    inner: HtmlInputElement,
}

impl Toggle {
    pub fn new(id: &str, document: &Document) -> Result<Self, JsValue> {
        let switch = document.create_element("label")?;
        switch.set_attribute("class", "switch")?;
        switch.set_attribute("id", id)?;
        let input = document.create_element("input")?;
        input.set_attribute("type", "checkbox")?;
        switch.append_child(&input)?;
        let span = document.create_element("span")?;
        span.set_attribute("class", "slider round")?;
        switch.append_child(&span)?;

        let result = Toggle {
            switch,
            inner: input.dyn_into().unwrap(),
        };
        Ok(result)
    }

    pub fn append_to_body(&self, body: &HtmlElement) -> Result<(), JsValue> {
        body.append_child(&self.switch)?;
        Ok(())
    }

    pub fn is_checked(&self) -> bool {
        self.inner.checked()
    }
}
