use yew::prelude::*;

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    html! {
        <div class="portfolio">
            <h1>{ "Portfolio" }</h1>
            <p class="description">
                { "I have created multiple projects for my portfolio. Below you can see some of my favorite projects. This is just a bunch of extra text that I'm putting here to see if it fills up the page :)" }
            </p>
        </div>
    }
}