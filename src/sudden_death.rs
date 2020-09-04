use std::cmp::max;
use url::Url;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

const DEFAULT_INPUT: &str = "突然の死";

fn char_width(c: char) -> usize {
    if c.is_ascii() {
        1
    } else {
        2
    }
}

fn sudden_death(s: &str) -> String {
    let s = if s.is_empty() {
        "\n".to_string()
    } else {
        s.to_string()
    };

    let mut width = 0;
    for line in s.lines() {
        let cur_width = line.chars().map(char_width).sum();
        width = max(width, cur_width);
    }
    if width % 2 == 1 {
        width += 1;
    }

    let mut ret = String::new();

    ret += "＿人";
    ret += &vec!['人'; width / 2].iter().collect::<String>();
    ret += "人＿\n";

    for line in s.lines() {
        ret += "＞　";

        let cur_width: usize = line.chars().map(char_width).sum();
        let mergin = width - cur_width;
        let left_mergin = mergin / 2;
        let right_mergin = mergin - left_mergin;

        for _ in 0..left_mergin / 2 {
            ret += "　";
        }
        if left_mergin % 2 == 1 {
            ret += "  ";
        }

        ret += line;

        for _ in 0..right_mergin / 2 {
            ret += "　";
        }
        if right_mergin % 2 == 1 {
            ret += "  ";
        }

        ret += "　＜\n";
    }

    ret += "￣";
    let low_width = width * 13 / 2 / 8 + 3;
    for i in 0..low_width {
        ret += if i % 2 == 0 { "Y" } else { "^" };
    }
    ret += "￣\n";

    ret
}

pub struct Model {
    link: ComponentLink<Self>,
    input: NodeRef,
    output: NodeRef,
}

pub enum Msg {
    Input,
    Tweet,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            input: NodeRef::default(),
            output: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let input = self.input.cast::<HtmlTextAreaElement>().unwrap();
        let output = self.output.cast::<HtmlTextAreaElement>().unwrap();

        match msg {
            Msg::Input => {
                output.set_value(&sudden_death(&input.value()));
            }
            Msg::Tweet => {
                let text = output.value();
                let mut url = Url::parse("https://twitter.com/intent/tweet").unwrap();
                url.query_pairs_mut().append_pair("text", &text);
                web_sys::window()
                    .unwrap()
                    .open_with_url_and_target(url.as_str(), "_blank")
                    .unwrap();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let default_output = sudden_death(DEFAULT_INPUT);

        html! {
            <>
            <h1 class="title">{"突然の死ジェネレーター"}</h1>

            <div class="field">
                <label class="label">{"入力テキスト"}</label>
                <div class="control">
                    <textarea class="textarea" placeholder=DEFAULT_INPUT
                        oninput=self.link.callback(|_| Msg::Input) ref=self.input.clone() />
                </div>
            </div>

            <div class="field">
                <label class="label">{"生成テキスト"}</label>
                <div class="control">
                    <textarea class="textarea" ref=self.output.clone()>
                        {default_output}
                    </textarea>
                </div>
            </div>

            <button class="button is-info" onclick=self.link.callback(|_| Msg::Tweet)>{"ツイート"}</button>

            </>
        }
    }
}
