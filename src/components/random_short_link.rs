use stylist::yew::styled_component;
use yew::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use web_sys::{window};

fn generate() -> String {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    string    
}

#[derive(Properties, PartialEq)]
pub struct RandomShortLinkProps {
    pub generated_url: String,
}

#[styled_component]
pub fn RandomShortLink(props: &RandomShortLinkProps) -> Html {
    let string = &props.generated_url;
    let domain = "https://mzch.in/";
    let url = format!("{}{}", domain, string);
    
    let _url = url.clone();
    
    let onclick = move |_: MouseEvent| {
        let w = window().expect("No Window Found");
        let navigator = w.navigator();
        let _ = navigator.clipboard().write_text(_url.as_str());
    };
    html! {
        <div>
            <p class={css!("
                color: #131313;
                font-family: MontserratBold;
                font-size: 18px;
                margin-block: 10px;
            ")}>{"Short Link"}</p>
            <div class={css!("
                background-color: #369;
                padding: 10px 15px;
                border-left: 15px solid #036;
                border-radius: 15px;
                display: flex;
                justify-content: space-between;
            ")}>
                <a class={css!("
                    text-decoration: none;
                    color: white;
                    letter-spacing: 1px;
                ")} href={url.clone()} rel={"noopener noreferrer"}>{url}</a>
                <span class={css!("
                    color: white;
                    cursor: pointer;
                ")} onclick={onclick}>{"copy"}</span>
            </div>
        </div>
    }
}