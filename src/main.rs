mod colors;
mod components;
mod utils;

use std::future::Future;
use crate::colors::{SVG_DEFAULT, SVG_GRADIENTS};
use crate::components::color_selector::ColorSelector;
use crate::components::random_short_link::RandomShortLink;
use crate::components::spacer::Spacer;
use qrcode_generator::QrCodeEcc;
use stylist::yew::styled_component;
use stylist::{Error, Style};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{window, Blob, BlobPropertyBag, HtmlElement, HtmlImageElement, HtmlTextAreaElement, Url, XmlSerializer, RequestCredentials, Headers};
use yew::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;

#[derive(Serialize)]
pub struct URLCreationRequest {
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct URLMapping {
    url: String,
    short_link: String,
}

#[wasm_bindgen]
pub async fn request_short_link(url: String) -> Result<String, JsValue> {
    let headers = Headers::new().unwrap();
    headers.set("Accept", "application/json").unwrap();
    headers.set("Content-Type", "application/json").unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&serde_wasm_bindgen::to_value(&serde_json::to_string(&URLCreationRequest { url: url.clone() }).unwrap()).unwrap()));
    opts.mode(RequestMode::Cors);
    opts.headers(&headers);
    opts.credentials(RequestCredentials::Include);


    let request = Request::new_with_str_and_init("https://mzch.in/create/", &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json().unwrap()).await;

    // Send the JSON response back to JS.
    match json {
        Ok(res) => {
            web_sys::console::log_1(&res);
            let result: String = js_sys::Reflect::get(&res, &"short_link".into())?.as_string().unwrap();
            Ok(result)
        }
        Err(e) => {
            web_sys::console::error_1(&e);
            Err(e)
        }
    }
}

#[styled_component]
fn App() -> Html {
    let color_default_style = Style::new(SVG_DEFAULT);
    let color_string = use_state(|| color_default_style);
    let color_string2 = color_string.clone();
    let div_ref = use_node_ref();
    let url = use_state(|| "".to_string());
    let url_c = (*url).clone();
    let div_ref3 = div_ref.clone();
    let short_link = use_state(|| "".to_string());

    let on_color_change = Callback::from({
        let color_string = color_string.clone();
        move |color_style: Result<Style, Error>| {
            color_string.set(color_style);
        }
    });

    let oninput = {
        let short_link = short_link.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input_elem: HtmlTextAreaElement = e.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            url.set(value.clone());
            let string_svg: String =
                qrcode_generator::to_svg_to_string(value.clone(), QrCodeEcc::Low, 300, None::<&str>)
                    .unwrap();
            let div = div_ref3
                .cast::<HtmlElement>()
                .expect("div_ref not attached to div element");
            let inject_gradients = SVG_GRADIENTS.to_string();

            let final_string = format!("{}{}", inject_gradients, string_svg);

            div.set_inner_html(final_string.as_str());

            spawn_local({
                let short_link = short_link.clone();
                async move {
                    let short_link_value: String = request_short_link(value).await.expect("No response from backend");
                    short_link.set(short_link_value);
                }
            });
        })
    };

    let _div_ref = div_ref.clone();
    let onclick = move |_: MouseEvent| {
        let div = _div_ref
            .cast::<HtmlElement>()
            .expect("_div_ref not attached to div element");
        let svg = div.children().item(0).unwrap();
        web_sys::console::log_1(&svg.children().item(2).expect("msg").to_string());
        let serializer = XmlSerializer::new().unwrap();
        let svg_string = serializer.serialize_to_string(&svg).unwrap();
        web_sys::console::log_1(&JsValue::from_str(&svg_string));
        let mut options = BlobPropertyBag::new();
        options.type_("image/svg+xml;charset=utf-8");
        let str_svg_as_js_value = JsValue::from_str(svg_string.as_str());
        let array = js_sys::Array::from_iter(std::iter::once(str_svg_as_js_value));
        let svg_blob = Blob::new_with_str_sequence(&array).expect("Blob creation failed.");
        let url = Url::create_object_url_with_blob(&svg_blob).expect("Url creation failed.");

        let image =
            HtmlImageElement::new_with_width_and_height(300, 300).expect("Image creation failed.");
        let image2 = image.clone();
        image.set_src(url.as_str());
        web_sys::console::log_1(&JsValue::from_bool(image.complete()));

        let img_load = move || {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.create_element("canvas").unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            web_sys::console::log_1(&context.to_string());

            let _ = context
                .draw_image_with_html_image_element(&image2, 0_f64, 0_f64)
                .expect("Drawing image failed.");
            let uri = canvas.to_data_url().expect("Image render failed.");

            let w = window().expect("Window not created");
            let new_tab = w
                .open()
                .expect("Failed to open window")
                .expect("Couldn't open window");
            new_tab
                .document()
                .expect("Couldn't get document")
                .body()
                .expect("Couldn't get body")
                .set_inner_html(
                    (format!("<img src=\"{}\" width=\"300px\" height=\"300px\">", uri)).as_str(),
                );
            web_sys::console::log_1(&JsValue::from_bool(image2.complete()));
            web_sys::console::log_1(&svg.children().item(2).expect("msg").to_string());
        };

        let wrapped_closure = Closure::wrap(Box::new(move || img_load()) as Box<dyn Fn()>);

        image.set_onload(Some(wrapped_closure.as_ref().unchecked_ref()));
    };

    html! {
        <main class={css!("
            width: 100%;
            height: 100vh;
            background-color: #212121;
            display: flex;
            justify-content: center;
            align-items: center;
        ")}>
            <div class={css!("
                width: 100%;
                height: 100%;
                background-color: #c8c8c8;
                padding: 30px;
            ")}>
                <h4 class={css!("
                    color: #131313;
                    font-family: MontserratLight;
                    font-size: 28px;
                    margin-block-end: 10px;
                ")}>
                    {"URL Shortner"}
                </h4>
                <p class={css!("
                    color: #131313;
                    font-family: Montserrat;
                    font-size: 14px;
                    margin-block: 10px;
                ")}>{"Enter URL to convert"}</p>
                <textarea rows={25} cols={50} class={css!("
                    width: 100%;
                    height: 90%;
                    /*padding-inline: 10px;*/
                    padding-block: 10px;
                    border: none;
                    border-radius: 5px;
                    background-color: #c8c8c8;
                    font-size: 32px;
                    resize: none;
                ")} placeholder={"Enter text, numbers, URLs, etc."} value={url_c} oninput={oninput}></textarea>
            </div>
            <div class={css!("
                width: 30vw;
                height: 100%;
                background-color: #e4e4e4;
                padding: 30px;
            
                > div#render {
                    width: 100%;
                    height: 50%;
                    background-color: #fff;
                    border-radius: 5px;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }
            ")}>
                <div id={"render"} class={(*((*color_string2).as_ref()).unwrap()).clone()} ref={div_ref}>

                </div>
                <Spacer vertical={15} />
                <RandomShortLink generated_url={(*short_link).clone()} />
                <Spacer vertical={15} />
                <ColorSelector on_color_change={on_color_change} />
                // <button class={css!("
                //     padding: 15px 30px;
                //     border-radius: 5px;
                //     border: none;
                //     background: #d1d1d1;
                //     position: absolute;
                //     right: 205px;
                //     bottom: 30px;
                // ")}>{"Create short link"}</button>
                // <button disabled={true} onclick={onclick} class={css!("
                //     padding: 15px 30px;
                //     border-radius: 5px;
                //     border: none;
                //     background: #d1d1d1;
                //     position: absolute;
                //     right: 30px;
                //     bottom: 30px;
                // ")}>{"Download as PNG"}</button>
                // <canvas id={"canvas"}></canvas>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
