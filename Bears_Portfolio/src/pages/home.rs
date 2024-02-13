use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home">
            <h1>{ "Welcome to Bear's Portfolio!" }</h1>
            <p class="description">
                { "Lifelong gamer and tech enthusiast with a proven track record in IT, esports, and technical communication. From building computers and Minecraft servers alongside my dad to competing in Overwatch esports, I've continuously refined my technical, leadership, and problem-solving skills. My experience at STEP honed my ability to research complex topics and break them down, while my time in esports developed team leadership and mentorship strengths.  Now, as a technical writer specializing in gaming and IT, I'm dedicated to translating complex concepts into clear, engaging documentation." }
            </p>
        </div>
    }
}