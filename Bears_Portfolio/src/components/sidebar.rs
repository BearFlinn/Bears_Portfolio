use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <nav class="sidebar">
            <h2>{ "Bear Flinn" }</h2>
            <menu class ="menu">
                <a class = "menu-item" href="/">{ "Home" }</a>
                <a class = "menu-item" href="/portfolio">{ "Portfolio" }</a>
                <a class = "menu-item" href="/resume">{ "Resume" }</a>
                <a class = "menu-item" href="/playground">{ "Playground" }</a>
            </menu>
        </nav>
    }
}