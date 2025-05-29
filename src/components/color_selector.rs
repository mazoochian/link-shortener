use stylist::yew::styled_component;
use stylist::{Style, Error};
use yew::prelude::*;
use yew::Properties;
use crate::lib::colors::QRColor;

#[derive(Properties, PartialEq)]
pub struct ColorSelectorProps {
    pub on_color_change: Callback<Result<Style, Error>>
}

#[styled_component]
pub fn ColorSelector(props: &ColorSelectorProps) -> Html {
    let on_color_change = props.on_color_change.clone();
    let _on_color_change = Callback::from(move |qr_color: QRColor| {
        match qr_color {
            QRColor::Blue => {
                let new_style = Style::new("
                svg {
                    fill: #369;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Purple => {
                let new_style = Style::new("
                svg {
                    fill: #639;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Pink => {
                let new_style = Style::new("
                svg {
                    fill: #936;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Green => {
                let new_style = Style::new("
                svg {
                    fill: #693;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Orange => {
                let new_style = Style::new("
                svg {
                    fill: #ECB159;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Brown => {
                let new_style = Style::new("
                svg {
                    fill: #B67352;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Teal => {
                let new_style = Style::new("
                svg {
                    fill: #8CB9BD;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::Navy => {
                let new_style = Style::new("
                svg {
                    fill: #0C2D57;
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::GradBlue => {
                let new_style = Style::new("
                svg {
                    fill: url(#gradblue);
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
            QRColor::GradSeaSalt => {
                let new_style = Style::new("
                svg {
                    fill: url(#gradseasalt);
                }
                svg > rect {
                    display: none;  
                }
                ");
                on_color_change.emit(new_style);
            },
        }
    });
    html! {
        <div>
            <p class={css!("
                    color: #131313;
                    font-family: MontserratBold;
                    font-size: 18px;
                    margin-block: 10px;
            ")}>{"Choose QR Colour"}</p>
            <div class={css!("
                justify-content: center;
            
                div {
                    width: 50px;
                    height: 50px;
                    margin-inline-end: 10px;
                    border-radius: 50%;
                    margin-block-end: 10px;
                    display: inline-block;
                }
            ")}>
                <div class={css!("
                    background-color: #369;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Blue);}
                }></div>
                <div class={css!("
                    background-color: #639;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Purple);}
                }></div>
                <div class={css!("
                    background-color: #936;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Pink);}
                }></div>
                <div class={css!("
                    background-color: #693;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.emit(QRColor::Green);}
                }></div>
                <div class={css!("
                    background-color: #ECB159;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Orange);}
                }></div>
                <div class={css!("
                    background-color: #B67352;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Brown);}
                }></div>
                <div class={css!("
                    background-color: #8CB9BD;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Teal);}
                }></div>
                <div class={css!("
                    background-color: #0C2D57;
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.clone().emit(QRColor::Navy);}
                }></div>
                <div class={css!("
                    background: linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(9,9,121,1) 35%, rgba(0,212,255,1) 100%);
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.emit(QRColor::GradBlue);}
                }></div>
                <div class={css!("
                    background: linear-gradient(90deg, #4b6cb7 0%, #182848 100%);
                ")} onclick={
                    let on_color_change = _on_color_change.clone();
                    move |_| {on_color_change.emit(QRColor::GradSeaSalt);}
                }></div>
            </div>
        </div>
    }
}
