use std::marker::PhantomData;
use yew::prelude::*;

pub struct TypedNodeRef<T>(pub NodeRef, PhantomData<T>);

impl<T> Default for TypedNodeRef<T> {
    fn default() -> Self {
        Self(Default::default(), PhantomData::default())
    }
}

impl<T: AsRef<web_sys::Node> + From<wasm_bindgen::JsValue>> TypedNodeRef<T> {
    pub fn node_ref(&self) -> NodeRef {
        self.0.clone()
    }

    pub fn get(&self) -> T {
        self.0.cast::<T>().unwrap()
    }
}

pub fn horizontal_field(label: &str, field: Html) -> Html {
    html! {
        <div class="field is-horizontal">
            <div class="field-label is-normal">
                <label class="label">{ label }</label>
            </div>
            <div class="field-body">
                <div class="field">
                    <div class="control">
                        { field }
                    </div>
                </div>
            </div>
        </div>
    }
}
