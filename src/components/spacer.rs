use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, PartialEq)]
pub struct SpacerProps {
    #[prop_or_default]
    pub vertical: usize,
    #[prop_or_default]
    pub horizontal: usize,
}

#[styled_component]
pub fn Spacer(props: &SpacerProps) -> Html {
    html! {
        <object class={css!("
            visibility: hidden;
        ")} width={props.horizontal.to_string()} height={props.vertical.to_string()}></object>
    }
}