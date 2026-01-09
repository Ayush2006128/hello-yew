use gloo_net::http::Request;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    // State values
    let a_val = use_state(|| 0.0f32);
    let b_val = use_state(|| 0.0f32);
    let method = use_state(|| "add".to_string());
    let result_display = use_state(|| "Ready to calculate".to_string());

    // Methods to update states
    let on_a_change = {
        let a_val = a_val.clone();
        let _ = Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<f32>() {
                a_val.set(val);
            }
        });
    };

    let on_b_change = {
        let b_val = b_val.clone();
        let _ = Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<f32>() {
                b_val.set(val);
            }
        });
    };

    let on_method_change = {
        let method = method.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            method.set(input.value());
        })
    };
    let on_calculate = {
        let a = *a_val;
        let b = *b_val;
        let m = (*method).clone();
        let result_display = result_display.clone();

        Callback::from(move |_| {
            let m = m.clone();
            let result_display = result_display.clone();

            // Spawn async task for the API call
            wasm_bindgen_futures::spawn_local(async move {
                // Construct URL: e.g., http://localhost:8000/calculate?method=add&a=10&b=20
                let url = format!("http://localhost:8000/calculate?method={}&a={}&b={}", m, a, b);

                match Request::get(&url).send().await {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            result_display.set(format!("Result: {}", text));
                        } else {
                            result_display.set("Error parsing response".into());
                        }
                    }
                    Err(_) => result_display.set("Failed to fetch from API".into()),
                }
            });
        })
    };

    html! {
        <div class="container">
            <article>
                <header>
                    <h3>{ "Yew + Rocket = Amazing!! 🚀" }</h3>
                </header>

                <label>{ "Number A" }</label>
                <input type="number" oninput={move |_| on_a_change} value={a_val.to_string()} />

                <label>{ "Number B" }</label>
                <input type="number" oninput={move |_| on_b_change} value={b_val.to_string()} />

                <label>{ "Operation" }</label>
                <select onchange={on_method_change}>
                    <option value="add">{ "Add (+)" }</option>
                    <option value="sub">{ "Subtract (-)" }</option>
                    <option value="mul">{ "Multiply (*)" }</option>
                    <option value="div">{ "Divide (/)" }</option>
                </select>

                <button onclick={on_calculate}>{ "Calculate" }</button>

                <div class="result-box">
                    <h2>{ (*result_display).clone() }</h2>
                </div>
            </article>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
