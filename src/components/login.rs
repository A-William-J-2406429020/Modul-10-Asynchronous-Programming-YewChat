use js_sys::Math;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let is_dark = use_state(|| true);
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone();
        })
    };

    let randomize = {
        let username = username.clone();
        Callback::from(move |_| {
            let choices = [
                "StarPilot",
                "CozyCoder",
                "LunarLoop",
                "PixelDrift",
                "NovaNote",
                "CloudKick",
            ];
            let index = (Math::random() * choices.len() as f64).floor() as usize;
            username.set(choices[index].to_string());
        })
    };

    let toggle_theme = {
        let is_dark = is_dark.clone();
        Callback::from(move |_| {
            is_dark.set(!*is_dark);
        })
    };

    let background_class = if *is_dark {
        "bg-gray-800"
    } else {
        "bg-gray-100"
    };

    let button_class = if *is_dark {
        "px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r"
    } else {
        "px-8 rounded-r-lg bg-emerald-500 text-white font-bold p-4 uppercase border-emerald-500 border-t border-b border-r"
    };

    html! {
        <div class={classes!("flex", "w-screen", "h-screen", background_class)}>
            <div class="container mx-auto flex flex-col justify-center items-center">
                <form class="m-4 flex flex-wrap gap-2 items-center">
                    <input
                        {oninput}
                        value={(*username).clone()}
                        class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white"
                        placeholder="Username..."
                    />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class={button_class}>
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                    <button
                        type="button"
                        onclick={randomize}
                        class="px-4 py-3 rounded-lg border border-gray-300 bg-white text-gray-700"
                    >
                        {"Random"}
                    </button>
                    <button
                        type="button"
                        onclick={toggle_theme}
                        class="px-4 py-3 rounded-lg border border-gray-300 bg-white text-gray-700"
                    >
                        {"Ganti tema"}
                    </button>
                </form>
            </div>
        </div>
    }
}