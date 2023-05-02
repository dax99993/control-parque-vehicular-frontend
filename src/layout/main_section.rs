use yew::prelude::*;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MainSectionProps {
    pub children: Children,
    pub route: String,
    pub subroute: String,
    pub title: String,
}

#[function_component]
pub fn MainSection(props: &MainSectionProps) -> Html {
    let props = props.clone();
    html! {
        <>
            <TitleBarSection route={ props.route } subroute={ props.subroute }/>
            <HeroBarSection title={ props.title }/>
            <section class="section is-main-section">
                { props.children.clone() }
            </section>
        </>
    }
}

#[function_component]
fn TitleBarSection(props: &TitleBarSectionProps) -> Html {
    let props = props.clone();
    html! {
        <section class="section is-title-bar">
            <div class="level">
                <div class="level-left">
                    <div class="level-item">
                        <ul>
                            <li>{props.route}</li>
                            <li>{props.subroute}</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TitleBarSectionProps {
    pub route: String,
    pub subroute: String,
}

#[function_component]
fn HeroBarSection(props: &HeroBarSectionProps) -> Html {
    let props = props.clone();

    html! {
        <section class="hero is-hero-bar">
            <div class="hero-body">
                <div class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <h1 class="title">
                                { props.title }
                            </h1>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct HeroBarSectionProps {
    pub title: String,
}
