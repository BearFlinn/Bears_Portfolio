//footer.rs
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <div class="socials">
                <h2>{"Check out my socials:"}</h2>
                <SocialButtons />
            </div>
            <p class="copyright">{"Copyright © 2024 Bear Flinn. All rights reserved."}</p>
        </div>
    }
}

#[function_component(SocialButtons)]
pub fn social_buttons() -> Html {
    html! {
        <div class="social_buttons">
            <a class="sm_button" href="https://www.tiktok.com/@bearflinn" target="_blank">
                <i class="fab fa-tiktok social_icon"></i>
            </a>
            <a class="sm_button" href="https://twitter.com/TheBearFlinn" target="_blank">
                <i class="fab fa-twitter social_icon"></i>
            </a>
            <a class="sm_button" href="https://www.linkedin.com/in/bear-flinn/" target="_blank">
                <i class="fab fa-linkedin social_icon"></i>
            </a>
            <a class="sm_button" href="https://www.instagram.com/thebearflinn/" target="_blank">
                <i class="fab fa-instagram social_icon"></i>
            </a>
        </div>
    }
}