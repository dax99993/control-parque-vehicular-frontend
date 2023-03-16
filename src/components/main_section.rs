use yew::prelude::*;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MainSectionProps {
    pub children: Children,
}

#[function_component]
pub fn MainSection(props: &MainSectionProps) -> Html {
    html! {
        <>
            <TitleBarSection privilege="Administrator" route="Some Route"/>
            <HeroBarSection title="Some Title"/>
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
                            <li>{"Admin"}</li>
                            <li>{props.route}</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TitleBarSectionProps {
    pub privilege: String,
    pub route: String,
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
