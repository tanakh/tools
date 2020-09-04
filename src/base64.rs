use crate::util::TypedNodeRef;
use base64::{decode, encode};
use web_sys::{HtmlAnchorElement, HtmlTextAreaElement};
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    encode_tab: TypedNodeRef<HtmlAnchorElement>,
    decode_tab: TypedNodeRef<HtmlAnchorElement>,
    input: TypedNodeRef<HtmlTextAreaElement>,
    output: TypedNodeRef<HtmlTextAreaElement>,
}

pub enum Msg {
    Encode,
    Decode,
    Input,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            encode_tab: Default::default(),
            decode_tab: Default::default(),
            input: Default::default(),
            output: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input => {
                let is_encode = self.encode_tab.get().class_list().contains("is-active");
                let s = self.input.get().value();

                if is_encode {
                    self.output.get().set_value(&encode(s));
                } else {
                    self.output.get().set_value(
                        &decode(s)
                            .map_err(|e| e.to_string())
                            .and_then(|v| String::from_utf8(v).map_err(|e| e.to_string()))
                            .unwrap_or_else(|e| format!("Error: {}", e)),
                    );
                }
            }
            Msg::Encode => {
                self.encode_tab.get().set_class_name("is-active");
                self.decode_tab.get().set_class_name("");

                let input_value = self.input.get().value();
                let output_value = self.output.get().value();
                self.input.get().set_value(&output_value);
                self.output.get().set_value(&input_value);
            }
            Msg::Decode => {
                self.encode_tab.get().set_class_name("");
                self.decode_tab.get().set_class_name("is-active");

                let input_value = self.input.get().value();
                let output_value = self.output.get().value();
                self.input.get().set_value(&output_value);
                self.output.get().set_value(&input_value);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="tabs is-boxed">
                <ul>
                    <li ref=self.encode_tab.node_ref() class="is-active">
                        <a onclick=self.link.callback(|_| Msg::Encode)>{"Encode"}</a>
                    </li>
                    <li ref=self.decode_tab.node_ref()>
                        <a onclick=self.link.callback(|_| Msg::Decode)>{"Decode"}</a>
                    </li>
                </ul>
            </div>

            <div class="field">
                <label class="label">{"Input"}</label>
                <div class="control">
                    <textarea class="textarea" oninput=self.link.callback(|_| Msg::Input) ref=self.input.node_ref() />
                </div>
            </div>
            <div class="field">
                <label class="label">{"Output"}</label>
                <div class="control">
                    <textarea class="textarea" ref=self.output.node_ref() readonly=true/>
                </div>
            </div>
            </>
        }
    }
}
