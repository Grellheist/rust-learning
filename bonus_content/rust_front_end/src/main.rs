use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 10,
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{ "click here to gain 10 dollars" }</button>
            <p>{ "This is how many you have: " }{ state.value }</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
