pub mod file_upload;

use file_upload::UploadFile;

use yew::prelude::*;
use yew_hooks::prelude::*;

use gloo::net::http::FormData;

#[derive(Debug, PartialEq, Properties)]
pub struct UploadProps {
    pub request_url: String,
}



#[function_component]
pub fn Upload(props: &UploadProps) -> Html {
    //let form = use_state(|| FormData::new)
    let send_files: UseAsyncHandle<(), ()> = {
        let url = props.request_url.clone();
        use_async(async move {
            let form = FormData::new().unwrap();
            //form.append_with_blob("")
            form.append_with_str("some name", "some data");
            log::debug!("upload form {:?}", form);
            return Ok(())
        })
    };

    let onclick = {
        let send_files = send_files.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            send_files.run();
        })
    };

    html! {
        <>
        <UploadFile accept={vec!["image/*".into()]} multiple={false} max_files={1}/>
        <button class="button is-info" {onclick}/>
        </>
    }
}
