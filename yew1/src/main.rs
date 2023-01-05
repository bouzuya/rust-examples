use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    html! {
        { "This component has no properties!" }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    user_first_name: String,
    user_last_name: String,
}

#[function_component]
fn MyComponentWithProps(props: &Props) -> Html {
    let Props {
        user_first_name,
        user_last_name,
    } = props;
    html! {
        <p>{"user_first_name: "}{user_first_name}{" and user_last_name: "}{user_last_name}</p>
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // <https://yew.rs/docs/concepts/html/components>

    let props = Props {
        user_first_name: "Bob".to_owned(),
        user_last_name: "Smith".to_owned(),
    };

    html! {
        <>
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>

            <MyComponent />

            <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

            <MyComponentWithProps ..props.clone() />

            <MyComponentWithProps user_first_name="Elm" ..props />

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
