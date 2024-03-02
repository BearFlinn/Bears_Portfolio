use yew::prelude::*;

//Props for the DisplayCard component
#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub link: Option<String>,
    pub link_is_external: Option<bool>,
    pub content: Option<Html>,
}

//Creates a card based on the provided props
#[function_component(DisplayCard)]
pub fn display_card(props: &CardProps) -> Html {
    let title = props.title.clone();
    let link = props.link.clone();
    let content = props.content.clone();

    //If there is no link, render a static card
    if props.link.is_none() { 
        html! {
            <div class="static_card">
                <h3 class="title">{ title }</h3>
                <div class="content">{ content }</div>
            </div>
        }
    //If there is a link, render a clickable card
    } else {
        let target = if props.link_is_external.unwrap_or(false) { "_blank" } else { "_self" };
        html! {
            <a href={ link } target={ target } rel="noopener noreferrer">
                <div class="clickable_card">
                    <h3 class="title">{ title }</h3>
                    <div class="content"> { content } </div>
                </div>
            </a>
        }
    }
}

/// Props for the card layout component
#[derive(Properties, PartialEq)]
pub struct CardLayoutProps {
    pub children: Children,  // Children of the card layout
    pub layout: LayoutType,
}

/// Enum defining different layout types for the card layout
#[derive(Clone, PartialEq)]
pub enum LayoutType {
    Grid(usize),
    //Row,
    //Column,
}

/// Renders the card layout based on the provided props
#[function_component(CardLayout)]
pub fn card_layout(props: &CardLayoutProps) -> Html {
    let layout_class = match props.layout {
        LayoutType::Grid(columns) => format!("grid-cols-{}", columns),
        // LayoutType::Row => "flex-row".to_string(),
        // LayoutType::Column => "flex-col".to_string(),
    };

    html! {
        <div class = "card-layout">
            <div class= { layout_class }>
                { for props.children.iter() }
            </div>
        </div>
    }
}


