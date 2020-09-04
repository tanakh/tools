use crate::util::TypedNodeRef;
use num_bigint::BigUint;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const BASES: &[usize] = &[2, 8, 10, 16, 36];

pub struct Model {
    link: ComponentLink<Self>,
    text: Vec<TypedNodeRef<HtmlInputElement>>,
}

pub enum Msg {
    Input(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            text: (0..=36).map(|_| Default::default()).collect(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input(base) => {
                let text = self.text[base].get().value();
                self.text[base]
                    .get()
                    .class_list()
                    .remove_1("is-danger")
                    .unwrap();

                if text.is_empty() {
                    for &base in BASES.iter() {
                        self.text[base].get().set_value("");
                    }
                    return true;
                }

                let num = BigUint::parse_bytes(text.as_bytes(), base as u32);

                if num.is_none() {
                    self.text[base]
                        .get()
                        .class_list()
                        .add_1("is-danger")
                        .unwrap();
                    return true;
                }

                let num = num.unwrap();

                for &base in BASES.iter() {
                    self.text[base]
                        .get()
                        .set_value(&num.to_str_radix(base as u32));
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <>
            {
                BASES.iter().map(|&base| html!{
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{format!("base-{}", base)}</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <input class="input" type="text" placeholder="0"
                                        oninput=self.link.callback(move |_| Msg::Input(base))
                                        ref=self.text[base].node_ref()/>
                                </div>
                            </div>
                        </div>
                    </div>
                }).collect::<Html>()
            }
            </>
        }
    }
}
