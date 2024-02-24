use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SlideProps {
    pub id: i32,
    pub children: Children,
}

#[function_component(Slide)]
pub fn slide(props: &SlideProps) -> Html {
    let id = props.id;
    html! {
        <div class={format!("slide-{}", id)} id={format!("slide-{}", id)}>
            { for props.children.clone() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub children: ChildrenWithProps<Slide>,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let current_slide = use_state(|| 0);
    let slide_count = props.children.len();

    let visible_slide = props.children.iter().enumerate().find_map(|(index, slide)| {
        if index == *current_slide {Some(slide)} else {None}
    });

    let click_next = {
        let current_slide = current_slide.clone();
        let slide_count = slide_count;
        Callback::from(move |_| {
            let mut next_slide = *current_slide + 1;
            if next_slide >= slide_count {
                next_slide = 0;
            }
            current_slide.set(next_slide);
        })
    };

    let click_prev = {
        let current_slide = current_slide.clone();
        let slide_count = slide_count;
        Callback::from(move |_| {
            let prev_slide = if *current_slide == 0 {
                slide_count - 1
            } else {
                *current_slide - 1
            };
            current_slide.set(prev_slide);
        })
    };

    html! {
        <div class="carousel">
            <button class="carousel-button" onclick={click_next}>
                <i class="material-icons">{"chevron_right"}</i>
            </button>
            <div class="slides">
                {
                    if let Some(slide) = visible_slide {
                        html! { for slide.props.children.iter() }
                    } else {
                        html! { <h1> { "Loading..." } </h1> }
                    }
                }
            </div>
            <button class="carousel-button" onclick={click_prev}>
                <i class="material-icons">{"chevron_left"}</i>
            </button>
        </div>
    }
}
