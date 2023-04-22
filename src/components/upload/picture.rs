use yew::prelude::*;
//use yew_hooks::prelude::*;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use super::pictures::{Reducer, FileDetails, FileActions};


#[derive(Debug, PartialEq, Properties)]
pub struct PictureProps {
    pub pictures_dispatcher: UseReducerDispatcher<Reducer>,
    pub file_details: FileDetails,
}

#[function_component]
pub fn Picture(props: &PictureProps) -> Html {
    let file = props.file_details.clone();
    let pictures_dispatcher = props.pictures_dispatcher.clone();

    let cb = {
        let pictures_dispatcher = pictures_dispatcher.clone();
        let file = file.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            pictures_dispatcher.dispatch(FileActions::Remove(file.id.clone(), file.name.clone()));
        })
    };

    html!{
        <div class="card is-centered">
            <div class="card-header">
                <p class="card-header-title">{ format!("{}", file.name) }</p>
                <div class="card-header-icon">
                    <button class="delete" onclick={cb}>
                    </button>
                </div>
            </div>
            <div class="card-image">
                <figure class="image">
                if file.mime.contains("image") {
                    if file.bytes.is_some() {
                    <img src={format!("data:{};base64,{}", file.mime, STANDARD.encode(&file.bytes.unwrap()))} />
                    }
                }
                </figure>
            </div>
        </div>
    }
}
