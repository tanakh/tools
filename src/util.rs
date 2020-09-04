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
