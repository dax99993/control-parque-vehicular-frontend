use std::ops::Deref;

use yew::prelude::*;
use web_sys::HtmlElement;

use crate::shadow_clone;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PaginationProps {
    pub total_pages: usize,
    pub current_page_state: UseStateHandle<usize>,
}


#[function_component]
pub fn Pagination(props: &PaginationProps) -> Html {
    //shadow_clone!(props);
    let PaginationProps { total_pages, current_page_state } = props;
    let total_pages = total_pages.clone();
    let current_page = *current_page_state.deref();

    // Get click page button name and set it to parent current page state
    let onclick = {
        shadow_clone!(current_page_state);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let button: HtmlElement = e.target_unchecked_into();
            if let Some(name) = button.get_attribute("name") {
                if let Ok(page) = name.parse::<usize>() {
                    current_page_state.set(page);
                }
            }
        })
    };

    // Calculate appropiate page offset and page range to show 
    let offset: usize = if current_page == 1 || current_page == total_pages {
        2
    } else {
        1
    };
    let start_page = std::cmp::max(1, current_page as i32 - offset as i32) as usize;
    let end_page = std::cmp::min(total_pages, current_page + offset);


    // Create page buttons
    let buttons = (start_page..=end_page)
        .into_iter()
        .map(|page| {
            let active = page == current_page;
            html!(<button type="button" 
                  onclick={onclick.clone()}
                  name={page.to_string()}
                  class={classes!["button", active.then(|| "is-active")]}>
                    {page}
                  </button>)
        })
        .collect::<Vec<Html>>();
        
    // Create page information string
    let page_text = if total_pages == 0 { 
        format!("Pagina 1 de 1")
    }
    else if current_page > total_pages {
        format!("Pagina {} de {}", total_pages, total_pages)
    }
    else {
        format!("Pagina {} de {}", current_page, total_pages)
    };

        

    // HTML
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
