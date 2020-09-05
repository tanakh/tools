use std::mem::swap;
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or(true)]
    encode: bool,
    #[prop_or_default]
    input: String,
    #[prop_or_default]
    output: String,
    #[prop_or_default]
    error: bool,
}

pub enum Msg {
    Input(String),
    Encode(bool),
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(txt) => {
                if self.props.encode {
                    let output = urlencoding::encode(txt.as_ref());
                    self.props.input = txt;
                    self.props.output = output;
                    self.props.error = false;
                } else {
                    let output = urlencoding::decode(txt.as_ref());
                    self.props.input = txt;
                    if let Ok(output) = output {
                        self.props.output = output;
                        self.props.error = false;
                    } else {
                        self.props.error = true;
                    }
                }
            }
            Msg::Encode(b) => {
                if self.props.encode != b {
                    self.props.encode = b;
                    swap(&mut self.props.input, &mut self.props.output);
                }
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
                    <li class={
                        let mut classes = vec![];
                        if self.props.error { classes.push("is-danger"); }
                        if self.props.encode { classes.push("is-active"); }
                        classes
                    }>
                        <a onclick=self.link.callback(|_| Msg::Encode(true))>{"Encode"}</a>
                    </li>
                    <li class=if self.props.encode {""} else {"is-active"}>
                        <a onclick=self.link.callback(|_| Msg::Encode(false))>{"Decode"}</a>
                    </li>
                </ul>
            </div>

            <div class="field">
                <label class="label">{"Input"}</label>
                <div class="control">
                    <textarea class="textarea"
                        value=self.props.input
                        oninput=self.link.callback(|e: InputData| Msg::Input(e.value))/>
                </div>
            </div>
            <div class="field">
                <label class="label">{"Output"}</label>
                <div class="control">
                    <textarea class="textarea" value=self.props.output readonly=true/>
                </div>
            </div>
            </>
        }
    }
}
