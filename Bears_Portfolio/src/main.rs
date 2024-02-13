mod pages;
mod components;

use components::*;
use pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Page {
    #[at("/")]
    Home,
    #[at("/portfolio")]
    Portfolio,
    #[at("/resume")]
    Resume,
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <div class="app">
        <Sidebar />
        <main class="pages">
        <BrowserRouter>
                <Switch<Page> render={switch} />
        </BrowserRouter>
        </main>
    </div>
    }
}

fn switch(page: Page) -> Html {
    match page {
        Page::Home => html! { <Home /> },
        Page::Portfolio => html! { <Portfolio /> },
        Page::Resume => html! { <Resume /> },
        _ => html! { 
            <div> { "404" } </div>},
    }
}

pub fn main() {
    yew::Renderer::<App>::new().render();
}