use grex::{Feature, RegExpBuilder};
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    input: NodeRef,
    output: NodeRef,

    digit: Rc<RefCell<bool>>,
    non_digit: Rc<RefCell<bool>>,
    space: Rc<RefCell<bool>>,
    non_space: Rc<RefCell<bool>>,
    word: Rc<RefCell<bool>>,
    non_word: Rc<RefCell<bool>>,
    repetition: Rc<RefCell<bool>>,
    case_insensitivity: Rc<RefCell<bool>>,
    capturing_group: Rc<RefCell<bool>>,
}

pub enum Msg {
    Input,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: Default::default(),
            output: Default::default(),
            digit: Default::default(),
            non_digit: Default::default(),
            space: Default::default(),
            non_space: Default::default(),
            word: Default::default(),
            non_word: Default::default(),
            repetition: Default::default(),
            case_insensitivity: Default::default(),
            capturing_group: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let input = self.input.cast::<HtmlTextAreaElement>().unwrap();
        let output = self.output.cast::<HtmlTextAreaElement>().unwrap();

        match msg {
            Msg::Input => {
                let s = input.value();
                let ls = s.lines().collect::<Vec<_>>();

                if ls.is_empty() {
                    output.set_value("");
                } else {
                    let mut feats = vec![];

                    let fs = vec![
                        (&self.digit, Feature::Digit),
                        (&self.non_digit, Feature::NonDigit),
                        (&self.space, Feature::Space),
                        (&self.non_space, Feature::NonSpace),
                        (&self.word, Feature::Word),
                        (&self.non_word, Feature::NonWord),
                        (&self.repetition, Feature::Repetition),
                        (&self.case_insensitivity, Feature::CaseInsensitivity),
                        (&self.capturing_group, Feature::CapturingGroup),
                    ];

                    for (f, v) in fs {
                        if *f.borrow() {
                            feats.push(v);
                        }
                    }

                    let mut builder = RegExpBuilder::from(&ls);
                    let builder = if feats.is_empty() {
                        &mut builder
                    } else {
                        builder.with_conversion_of(&feats)
                    };

                    let regexp = builder.build();
                    output.set_value(&regexp);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let opt = |checked: &Rc<RefCell<bool>>, name: &str| {
            html! {
                <>
                <label class="checkbox">
                    <Checkbox checked=checked.clone() onchange=self.link.callback(|_| Msg::Input)/>
                    { format!(" {}", name) }
                </label>
                {" "}
                </>
            }
        };

        html! {
            <>
            <div class="field">
                <label class="label">{"Input (line-separated list of strings)"}</label>
                <div class="control">
                    <textarea class="textarea" oninput=self.link.callback(|_| Msg::Input) ref=self.input.clone()/>
                </div>
            </div>

            <div class="field">
                <label class="label">{"Features"}</label>
                <div class="control">
                    { opt(&self.digit, "Digit") }
                    { opt(&self.non_digit, "Non digit") }
                    { opt(&self.space, "Space") }
                    { opt(&self.non_space, "Non space") }
                    { opt(&self.word, "word") }
                    { opt(&self.non_word, "Non word") }
                    { opt(&self.repetition, "Repetition") }
                    { opt(&self.case_insensitivity, "Case insensitivity") }
                    { opt(&self.capturing_group, "Capturing group") }
                </div>
            </div>


            <div class="field">
                <label class="label">{"Output (Regex)"}</label>
                <div class="control">
                    <textarea class="textarea" ref=self.output.clone() readonly=true/>
                </div>
            </div>
            </>
        }
    }
}

struct Checkbox {
    link: ComponentLink<Self>,
    props: CheckboxProps,
}

#[derive(Properties, Clone)]
struct CheckboxProps {
    checked: Rc<RefCell<bool>>,
    onchange: Callback<bool>,
}

enum CheckboxMsg {
    Click,
}

impl Component for Checkbox {
    type Message = CheckboxMsg;
    type Properties = CheckboxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CheckboxMsg::Click => {
                let mut r = self.props.checked.borrow_mut();
                *r = !*r;
                self.props.onchange.emit(*r);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="checkbox"
                checked=*self.props.checked.borrow()
                onclick=self.link.callback(|_| CheckboxMsg::Click)
            />
        }
    }
}
