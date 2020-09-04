use crate::util::TypedNodeRef;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
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
    text: TypedNodeRef<HtmlTextAreaElement>,
    md5: TypedNodeRef<HtmlInputElement>,
    sha1: TypedNodeRef<HtmlInputElement>,
    sha224: TypedNodeRef<HtmlInputElement>,
    sha256: TypedNodeRef<HtmlInputElement>,
    sha384: TypedNodeRef<HtmlInputElement>,
    sha512: TypedNodeRef<HtmlInputElement>,
    sha512_224: TypedNodeRef<HtmlInputElement>,
    sha512_256: TypedNodeRef<HtmlInputElement>,
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
            text: Default::default(),
            md5: Default::default(),
            sha1: Default::default(),
            sha224: Default::default(),
            sha256: Default::default(),
            sha384: Default::default(),
            sha512: Default::default(),
            sha512_224: Default::default(),
            sha512_256: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input => {
                let text = self.text.get().value();
                let text = text.as_bytes();
                self.md5.get().set_value(&digest_str::<md5::Md5>(text));
                self.sha1.get().set_value(&digest_str::<sha1::Sha1>(text));
                self.sha224
                    .get()
                    .set_value(&digest_str::<sha2::Sha224>(text));
                self.sha256
                    .get()
                    .set_value(&digest_str::<sha2::Sha256>(text));
                self.sha384
                    .get()
                    .set_value(&digest_str::<sha2::Sha384>(text));
                self.sha512
                    .get()
                    .set_value(&digest_str::<sha2::Sha512>(text));
                self.sha512_224
                    .get()
                    .set_value(&digest_str::<sha2::Sha512Trunc224>(text));
                self.sha512_256
                    .get()
                    .set_value(&digest_str::<sha2::Sha512Trunc256>(text));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hashes = [
            (&self.md5, "MD5"),
            (&self.sha1, "SHA-1"),
            (&self.sha224, "SHA-224"),
            (&self.sha256, "SHA-256"),
            (&self.sha384, "SHA-384"),
            (&self.sha512, "SHA-512"),
            (&self.sha512_224, "SHA-512/224"),
            (&self.sha512_256, "SHA-512/256"),
        ];

        html! {
            <>
            <div class="field">
                <label class="label">{"Text"}</label>
                <div class="control">
                    <textarea class="textarea" oninput=self.link.callback(|_| Msg::Input) ref=self.text.node_ref() />
                </div>
            </div>

            {
                hashes.iter().map(|(hash, title)| html! {
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{title}</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <input class="input" type="text" ref=hash.node_ref() readonly=true/>
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
