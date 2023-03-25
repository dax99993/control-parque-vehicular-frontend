use yew::prelude::*;
use web_sys::HtmlElement;

use crate::shadow_clone;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PaginationProps {
    //pub children: Children,
    pub total_pages: usize,
}


#[function_component]
pub fn Pagination(props: &PaginationProps) -> Html {
    let active_page = use_state(|| 1);

    let onclick = {
        shadow_clone!(active_page);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let button: HtmlElement = e.target_unchecked_into();
            log::debug!("Click on page {:?}", button);
            if let Some(name) = button.get_attribute("name") {
                log::debug!("button name {:?}", name);
                if let Ok(page) = name.parse::<usize>() {
                    active_page.set(page);
                }
            }
        })
    };

    shadow_clone!(props);
    let page_text = format!("Pagina {} de {}", *active_page, props.total_pages);

    let buttons = (1..=props.total_pages)
        .into_iter()
        .map(|i| {
            if i == *active_page {
                html!(<button type="button" class="button is-active" onclick={onclick.clone()} name={ i.to_string() }>{ i }</button>)
            } else {
                html!(<button type="button" class="button" onclick={onclick.clone()} name={ i.to_string() }>{ i }</button>)
            }
        })
        .collect::<Vec<Html>>();
        

    html!{
        <div class="notification">
            <div class="level">
                <div class="level-left">
                    <div class="level-item">
                        <div class="buttons has-addons">
                            { buttons }
                        </div>
                    </div>
                </div>
                <div class="level-right">
                    <div class="level-item">
                        <small>{ page_text }</small>
                    </div>
                </div>
            </div>
        </div>
    }
}
