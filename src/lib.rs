#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod base64;
mod base_converter;
mod digest;
mod regex;
mod sudden_death;
mod util;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="display:flex;min-height:100vh;flex-direction:column;">

            <nav class="navbar has-shadow is-spaced">
                <div class="navbar-brand">
                    <RouterAnchor<AppRoute> classes="navbar-item" route=AppRoute::Index>
                        <h1 class="title is-3">{"SUGOI Tools"}</h1>
                    </RouterAnchor<AppRoute>>
                </div>

                <div class="navbar-end">
                <a class="bd-navbar-icon navbar-item" href="https://github.com/tanakh/tools" target="_blank">
                  <span class="icon" style="color: #333;">
                    <i class="fab fa-lg fa-github-alt"></i>
                  </span>
                </a>

                <a class="bd-navbar-icon navbar-item" href="https://twitter.com/tanakh" target="_blank">
                  <span class="icon" style="color: #55acee;">
                    <i class="fab fa-lg fa-twitter"></i>
                  </span>
                </a>
                </div>
            </nav>

            <section class="section" style="flex:1;">
                <div class="columns">
                    <div class="column is-one-quarter">
                        <aside class="menu">
                            <p class="menu-label">
                                {"Tools"}
                            </p>
                            <ul class="menu-list">
                                <li><RouterLink text="Base64" route=AppRoute::Base64/></li>
                                <li><RouterLink text="Message digest (MD5, SHA-1, SHA-2)" route=AppRoute::Digest/></li>
                                <li><RouterLink text="Base converter" route=AppRoute::BaseConverter/></li>
                            </ul>

                            <p class="menu-label">
                                {"Generators"}
                            </p>
                            <ul class="menu-list">
                                <li><RouterLink text="Regex Generator" route=AppRoute::Regex/></li>
                                <li><RouterLink text="突然の死ジェネレーター" route=AppRoute::SuddenDeath/></li>
                            </ul>

                            <p class="menu-label">
                                {"Underconstructions"}
                            </p>
                            <ul class="menu-list">
                                <li><a>{"Character counter"}</a></li>
                                <li><a>{"ASCII converter"}</a></li>
                                <li><a>{"Prime factorization"}</a></li>
                            </ul>

                        </aside>
                    </div>

                    <div class="column">
                        <Router<AppRoute, ()>
                            render = Router::render(move |s| {
                                match s {
                                    AppRoute::Index => html!{<IndexModel/>},
                                    AppRoute::Base64 => html!{<crate::base64::Model/>},
                                    AppRoute::Digest => html!{<crate::digest::Model/>},
                                    AppRoute::BaseConverter => html!{<crate::base_converter::Model/>},
                                    AppRoute::Regex=>html!{<crate::regex::Model/>},
                                    AppRoute::SuddenDeath => html!{<crate::sudden_death::Model/>},
                                }
                            })
                        />
                    </div>
                </div>
            </section>

            <footer class="footer">
                <div class="content has-text-centered">
                <p>
                    {"Copyright (c) 2020, Hideyuki Tanaka"}
                </p>
                </div>
            </footer>

            </div>
        }
    }
}

struct RouterLink {
    props: RouteLinkProp,
}

#[derive(Properties, Clone)]
struct RouteLinkProp {
    text: String,
    route: AppRoute,
}

impl Component for RouterLink {
    type Message = ();
    type Properties = RouteLinkProp;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let route = self.props.route.clone();
        let text = self.props.text.clone();
        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |s: AppRoute| {
                    html!{
                        <RouterAnchor<AppRoute> route=route.clone()
                            classes=if s == route {"is-active"} else {""}>
                            {text.clone()}
                        </RouterAnchor<AppRoute>>
                    }
                })
            />
        }
    }
}

#[derive(Switch, Clone, PartialEq, Debug)]
enum AppRoute {
    #[to = "/static/#/base64"]
    Base64,
    #[to = "/static/#/digest"]
    Digest,
    #[to = "/static/#/base-conv"]
    BaseConverter,

    #[to = "/static/#/regex"]
    Regex,
    #[to = "/static/#/sudden-death"]
    SuddenDeath,

    #[to = "/static/"]
    Index,
}

struct IndexModel {}

impl Component for IndexModel {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h2 class="title is-2">
                {"👈(´･_･`👈) Select from here"}
            </h2>
        }
    }
}
