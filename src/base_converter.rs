use crate::util::horizontal_field;
use num_bigint::BigUint;
use yew::prelude::*;

const BASES: &[u32] = &[2, 8, 10, 16, 36];

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Input { base: u32, text: String },
    Uppercase,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    uppercase: bool,
    #[prop_or_default]
    value: Option<BigUint>,
    #[prop_or_default]
    error: Option<u32>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input { base, text } => {
                if text.is_empty() {
                    self.props.value = None;
                } else if let Some(num) = BigUint::parse_bytes(text.as_bytes(), base) {
                    self.props.value = Some(num);
                    self.props.error = None;
                } else {
                    self.props.error = Some(base);
                }
            }
            Msg::Uppercase => {
                self.props.uppercase = !self.props.uppercase;
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
                for BASES.iter().map(move |&base| {
                    let mut classes = vec!["input"];
                    if self.props.error == Some(base) {
                        classes.push("is-danger");
                    }
                    let value = self.props.value.clone().map_or("".to_string(), |n| n.to_str_radix(base));
                    let value = if self.props.uppercase {
                        value.to_uppercase()
                    } else {
                        value
                    };
                    horizontal_field(&format!("base-{}", base), html!{
                        <input class=classes type="text" placeholder="0"
                            value=value
                            oninput=self.link.callback(move |e: InputData| Msg::Input{base, text: e.value})/>
                    })
                })
            }
            {
                horizontal_field("", html!{
                    <label class="checkbox">
                        <input type="checkbox"
                            checked=self.props.uppercase
                            onchange=self.link.callback(|e: ChangeData| {
                                Msg::Uppercase
                            })/>
                        {" Uppercase"}
                    </label>
                })
            }
            </>
        }
    }
}
