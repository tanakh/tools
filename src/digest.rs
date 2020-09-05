use crate::util::{horizontal_field, TypedNodeRef};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

fn digest_str<T: digest::Digest>(r: &[u8]) -> String {
    let hash = T::digest(r);
    let mut ret = String::new();
    for c in hash {
        ret += &format!("{:02x}", c);
    }
    ret
}

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
    text: TypedNodeRef<HtmlTextAreaElement>,
}

pub enum Msg {
    Input,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    md5: String,
    #[prop_or_default]
    sha1: String,
    #[prop_or_default]
    sha224: String,
    #[prop_or_default]
    sha256: String,
    #[prop_or_default]
    sha384: String,
    #[prop_or_default]
    sha512: String,
    #[prop_or_default]
    sha512_224: String,
    #[prop_or_default]
    sha512_256: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            text: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input => {
                let text = self.text.get().value();
                let text = text.as_bytes();

                self.props.md5 = digest_str::<md5::Md5>(text);
                self.props.sha1 = digest_str::<sha1::Sha1>(text);
                self.props.sha224 = digest_str::<sha2::Sha224>(text);
                self.props.sha256 = digest_str::<sha2::Sha256>(text);
                self.props.sha384 = digest_str::<sha2::Sha384>(text);
                self.props.sha512 = digest_str::<sha2::Sha512>(text);
                self.props.sha512_224 = digest_str::<sha2::Sha512Trunc224>(text);
                self.props.sha512_256 = digest_str::<sha2::Sha512Trunc256>(text);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let input = |s| {
            html! {
                <input class="input" type="text" value=s readonly=true/>
            }
        };

        html! {
            <>
            <div class="field">
                <label class="label">{"Text"}</label>
                <div class="control">
                    <textarea class="textarea" oninput=self.link.callback(|_| Msg::Input) ref=self.text.node_ref() />
                </div>
            </div>

            { horizontal_field("MD5", input(&self.props.md5)) }
            { horizontal_field("SHA-1", input(&self.props.sha1)) }
            { horizontal_field("SHA-224", input(&self.props.sha224)) }
            { horizontal_field("SHA-256", input(&self.props.sha256)) }
            { horizontal_field("SHA-384", input(&self.props.sha384)) }
            { horizontal_field("SHA-512", input(&self.props.sha512)) }
            { horizontal_field("SHA-512/224", input(&self.props.sha512_224)) }
            { horizontal_field("SHA-512/256", input(&self.props.sha512_256)) }
            </>
        }
    }
}
