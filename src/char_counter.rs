use crate::util::TypedNodeRef;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
    input: TypedNodeRef<HtmlTextAreaElement>,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    chars: usize,
    #[prop_or_default]
    words: usize,
    #[prop_or_default]
    lines: usize,
}

pub enum Msg {
    Input,
}

impl Component for Model {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            input: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input => {
                let txt = self.input.get().value();

                let mut chars = 0;
                let mut words = 0;
                let mut lines = 0;

                let mut prev_ws = true;

                for c in txt.chars() {
                    chars += 1;

                    if prev_ws && !c.is_whitespace() {
                        words += 1;
                    }
                    prev_ws = c.is_whitespace();

                    if c == '\n' {
                        lines += 1;
                    }
                }

                self.props.chars = chars;
                self.props.words = words;
                self.props.lines = lines;
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
            <div class="field">
                <label class="label">{"Input"}</label>
                <div class="control">
                    <textarea class="textarea" oninput=self.link.callback(|_| Msg::Input) ref=self.input.node_ref() />
                </div>
            </div>

            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    <label class="label">{"Chars"}</label>
                </div>
                <div class="field-body">
                    <div class="field">
                        <div class="control">
                            <input class="input" type="text" value=self.props.chars readonly=true/>
                        </div>
                    </div>
                </div>
            </div>
            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    <label class="label">{"Words"}</label>
                </div>
                <div class="field-body">
                    <div class="field">
                        <div class="control">
                            <input class="input" type="text" value=self.props.words readonly=true/>
                        </div>
                    </div>
                </div>
            </div>
            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    <label class="label">{"Lines"}</label>
                </div>
                <div class="field-body">
                    <div class="field">
                        <div class="control">
                            <input class="input" type="text" value=self.props.lines readonly=true/>
                        </div>
                    </div>
                </div>
            </div>

            </>
        }
    }
}
