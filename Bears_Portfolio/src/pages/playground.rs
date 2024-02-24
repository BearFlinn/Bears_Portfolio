use crate::components::display_card::{CardLayout, LayoutType, DisplayCard};
use crate::components::carousel::{Carousel, Slide};
use yew::prelude::*;

#[function_component(Playground)]
pub fn playground() -> Html {
    html! {
        <div class="playground">
            <h1>{ "Playground" }</h1>
            <p class="description">
                { "This is a playground page. I'll be adding more features here in the future!" }
            </p>
            <CardLayout layout={LayoutType::Grid(3)}>
                <DisplayCard title="Card 1" link={"https://www.google.com"} link_is_external={Some(true)} content={html!{"This is card 1, which has a link and a bunch of extra text to fill out the space and test how it hanles extra content"}} />
                <DisplayCard title="Card 2" link={None::<String>} link_is_external={None} content={html!{"This is card 2"}} />
                <DisplayCard title="Card 3" link={None::<String>} link_is_external={None} content={html!{"This is card 3"}} />
                <DisplayCard title="Card 4" link={None::<String>} link_is_external={None} content={html!{"This is card 4"}} />
                <DisplayCard title="Card 5" link={None::<String>} link_is_external={None} content={html!{"This is card 5"}} />
                <DisplayCard title="Card 6" link={None::<String>} link_is_external={None} content={html!{"This is card 6"}} />
            </CardLayout>
            <Carousel>
                <Slide id={1}>
                    <h1>{ "Slide 1" }</h1>
                </Slide>
                <Slide id={2}>
                    <h1>{ "Slide 2" }</h1>
                </Slide>
                <Slide id={3}>
                    <h1>{ "Slide 3" }</h1>
                </Slide>
            </Carousel>
        </div>
    }
}