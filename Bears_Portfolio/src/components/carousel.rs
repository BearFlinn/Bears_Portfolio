use std::rc::Rc;
use gloo::timers::future::TimeoutFuture;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SlideAnimationState {
    Active,
    Inactive(Direction),
    Entering(Direction),
    Exiting(Direction),
    Reset(Direction),
}

impl SlideAnimationState {
    fn as_str(&self) -> &str {
        match self {
            SlideAnimationState::Active => "active",
            SlideAnimationState::Inactive(direction) => {
                match direction {
                    Direction::Left => "inactive-left",
                    Direction::Right => "inactive-right",
                }
            }
            SlideAnimationState::Entering(direction) => {
                match direction {
                    Direction::Left => "entering-left",
                    Direction::Right => "entering-right",
                }
            }
            SlideAnimationState::Exiting(direction) => {
                match direction {
                    Direction::Left => "exiting-left",
                    Direction::Right => "exiting-right",
                }
            }
            SlideAnimationState::Reset(direction) => {
                match direction {
                    Direction::Left => "reset-left",
                    Direction::Right => "reset-right",
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct CarouselState {
    active_slide: usize,
    slide_count: usize,
    animation_state: SlideAnimationState,
}

enum CarouselAction {
    NextSlide,
    PrevSlide,
    SetAnimationState(SlideAnimationState),
}

impl CarouselState {
    fn next_slide(&self) -> usize {
        (self.active_slide + 1) % self.slide_count
    }
    fn prev_slide(&self) -> usize {
        if self.active_slide == 0 {
            self.slide_count - 1
        } else {
            self.active_slide - 1
        }
    }
}

impl Reducible for CarouselState {
    type Action = CarouselAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CarouselAction::NextSlide => Rc::new(Self {
                active_slide: self.next_slide(),
                ..*self
            }),
            CarouselAction::PrevSlide => Rc::new(Self {
                active_slide: self.prev_slide(),
                ..*self
            }),
            CarouselAction::SetAnimationState(state) => Rc::new(Self {
                animation_state: state,
                ..*self
            })
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SlideProps {
    pub id: usize,
    pub children: Children,
}

#[function_component(Slide)]
pub fn slide(props: &SlideProps) -> Html {
    let carousel_state = use_context::<CarouselState>().expect("CarouselState not found");
    html! {
        <div class={format!("slide-{}", carousel_state.animation_state.as_str())}>
            <div class={format!("slide-{}", props.id)}>
                { for props.children.clone() }
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CarouselProps {
    pub children: ChildrenWithProps<Slide>,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let carousel_state = use_reducer(|| CarouselState {
        active_slide: 0,
        slide_count: props.children.len(),
        animation_state: SlideAnimationState::Active,
    });

    let delay = 300;

    let click_next: Callback<MouseEvent> = {
        let carousel_state = carousel_state.clone();
        Callback::from(move |_| {
            let carousel_state = carousel_state.clone();
            spawn_local(async move {
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Exiting(Direction::Left)));
                TimeoutFuture::new(delay).await;
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Inactive(Direction::Left)));
                TimeoutFuture::new(delay/4).await;
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Reset(Direction::Left)));
                TimeoutFuture::new(delay).await;
                carousel_state.dispatch(CarouselAction::NextSlide);
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Entering(Direction::Right)));
                TimeoutFuture::new(delay).await;
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Active));
            })
        })
    };

    let click_prev: Callback<MouseEvent> = {
        let carousel_state = carousel_state.clone();
        Callback::from(move |_| {
            let carousel_state = carousel_state.clone();
            spawn_local(async move {
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Exiting(Direction::Right)));
                TimeoutFuture::new(delay).await;
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Inactive(Direction::Right)));
                TimeoutFuture::new(delay/4).await;
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Reset(Direction::Right)));
                TimeoutFuture::new(delay).await;
                carousel_state.dispatch(CarouselAction::PrevSlide);
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Entering(Direction::Left)));
                TimeoutFuture::new(delay).await;
                carousel_state.dispatch(CarouselAction::SetAnimationState(SlideAnimationState::Active));
            })
        })
    };

    let visible_slide = props.children.iter().enumerate().find_map(|(slide_id, slide)| {
        if slide_id == carousel_state.active_slide {
            Some( slide )
        } else {None}
    });

    html! {
        <div class="carousel">
            <button class="carousel-button" onclick={click_next}>
                <i class="material-icons">{"chevron_right"}</i>
            </button>
            <ContextProvider<CarouselState> context={(*carousel_state).clone()}> 
                <div class="slides">
                    { visible_slide }
                </div>
            </ContextProvider<CarouselState>>
            <button class="carousel-button" onclick={click_prev}>
                <i class="material-icons">{"chevron_left"}</i>
            </button>
        </div>
    }
}
