use yew::ServerRenderer;
use yew::prelude::*;
use yew_style::Style;

#[derive(PartialEq, Properties)]
struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn Button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style
                .clone()
                .with_defaults([
                    ("padding", "0.5rem")
                ])
                .to_string()
            }
        >
            {props.children.clone()}
        </button>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <Button
            id="button"
            style={[
                ("color", "white"),
                ("background-color",  "gray"),
                ("border", "1px solid black")
            ]}
        >
            {"Click me"}
        </Button>
    }
}

#[tokio::test]
async fn test() {
    let renderer = ServerRenderer::<App>::new();
    renderer.render().await;
}
