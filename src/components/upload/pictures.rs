use gloo::file::{callbacks::FileReader, File};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::shadow_clone;

use crate::types::multipart::{MultipartForm, MultipartPart};

use super::picture::Picture;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct FileDetails {
    pub id: Uuid,
    pub name: String,
    pub mime: String,
    //pub description: String,
    //pub path: Option<String>,
    pub bytes: Option<Vec<u8>>,
    pub started_upload: bool,
    pub uploaded: bool,
}

pub enum FileActions {
    Add(Uuid, String, String),
    Remove(Uuid, String),
    UploadStarted(Uuid),
    Uploaded(Uuid),
    Loaded(Uuid, Vec<u8>),
    Reset,
    //Move(usize, usize),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Reducer {
    pictures: Vec<FileDetails>,
    counter: i32,
}

impl Reducible for Reducer {
    type Action = FileActions;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut pictures = self.pictures.clone();
        let mut counter = self.counter;
        match action {
            FileActions::Add(id, name, mime) => {
                pictures.push( FileDetails {
                    id,
                    name,
                    mime,
                    bytes: None,
                    started_upload: false,
                    uploaded: false
                });
                counter += 1;
            }
            FileActions::Remove(id, name) => {
                //let mut pics = pictures.borrow_mut();
                pictures.retain(|p| !(p.id.eq(&id) && p.name.eq(&name)));
                counter -= 1;
            }
            FileActions::UploadStarted(id) => {
                for picture in &mut pictures {
                    if picture.id.eq(&id) {
                        picture.started_upload = true;
                        break;
                    }
                }
            }
            FileActions::Uploaded(id) => {
                for picture in &mut pictures {
                    if picture.id.eq(&id) {
                        picture.uploaded= true;
                        break;
                    }
                }
            }
            FileActions::Loaded(id, bytes) => {
                for mut picture in &mut pictures {
                    if picture.id.eq(&id) {
                        picture.bytes = Some(bytes);
                        break;
                    }
                }
            }
            FileActions::Reset => {
                pictures = vec![];
                counter = 0;
            }
        }

        Self { pictures, counter }.into()
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct PicturesProps {
    pub upload_form: UseStateHandle<Option<MultipartForm>>,
    #[prop_or(1)]
    pub max_files: usize,
    #[prop_or(vec![String::from("image/*")])]
    pub accept: Vec<String>,
    #[prop_or(true)]
    pub multiple: bool,
}


#[function_component]
pub fn Pictures(props: &PicturesProps) -> Html {
    
    let max_files = props.max_files;
    let accept = props.accept.clone();

    let upload_form = props.upload_form.clone();

    let pictures = use_reducer(Reducer::default);
    let drag_over = use_counter(0);
    let readers = use_mut_ref(Vec::<FileReader>::new);

    let on_image_select = {
        let pictures = pictures.clone();
        let readers = readers.clone();
        let accept = accept.clone();
        Callback::from(move |e: Event| {
            if pictures.pictures.len() < max_files {
                let input: HtmlInputElement = e.target_unchecked_into();
                let mut loaders = process_pictures(&pictures.dispatcher(), input.files(), accept.clone());
                while let Some(loader) = loaders.pop() {
                    (*readers).borrow_mut().push(loader);
                }
            }
        })
    };

    let on_image_drop = {
        let pictures = pictures.clone();
        let drag_over = drag_over.clone();
        let accept = accept.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(0);
            if pictures.pictures.len() < max_files {
                if let Some(input) = e.data_transfer() {
                    let mut loaders = process_pictures(&pictures.dispatcher(), input.files(), accept.clone());
                    while let Some(loader) = loaders.pop() {
                        (*readers).borrow_mut().push(loader);
                    }
                    if let Err(e) = input.clear_data() {
                        log::warn!("Unable to clear drag data: {:?}", e);
                    };
                };
            }
        })
    };

    let on_drag_enter = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(input) = e.data_transfer() {
                if validate_list(&input.items()).is_none() {
                    return;
                }
                drag_over.increase();
            };
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                if validate_list(&input.items()).is_none() {
                    return;
                }
                drag_over.decrease();
            };
        })
    };

    let file_picker = use_node_ref();
    /*
    let f_picker = file_picker.clone();
    let click_add_image = Callback::from(move |_| {
        if let Some(element) = f_picker.cast::<HtmlInputElement>() {
            element.click();
        };
    });
    */

    let pics = pictures.pictures.clone();
    let on_start_upload = {
        shadow_clone![pics, pictures];
        shadow_clone!(upload_form);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let mut multipart = MultipartForm::default();
            for file in pics.clone() {
                if file.bytes.is_some() && !file.started_upload {
                    let bytes = file.bytes.unwrap().clone();
                    let name = file.name.clone();
                    let mime = file.mime.clone();
                    let id = file.id.clone();
                    pictures.dispatch(FileActions::UploadStarted(id));
                    let part = MultipartPart { bytes, name, mime };
                    multipart.append(part);
                }
            }
            log::debug!("multipart form\n{:?}", multipart);
            upload_form.set(Some(multipart));
        })
    };



    html!{
            <div class="container">
            <div class="columns is-centered has-text-centered">
            <div class="card column is-half">
                <div class={classes!["card-content", if pics.len() == 0 { "is-hidden" } else {""}]}>
                    { 
                        for pics.iter().map(|p| {
                            html! {
                                <Picture pictures_dispatcher={pictures.dispatcher()} file_details={p.clone()}/>
                            }
                        })
                    }
                </div>
                <div class="card-footer has-background-grey-lighter">
                    <div class="column"
                            ondrop={on_image_drop}
                            ondragover={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                            ondragenter={on_drag_enter}
                            ondragleave={on_drag_leave}
                            onchange={on_image_select}
                    >
                        <p>{ "Haz click en el boton o arrastra el archivo" }</p>
                        <i class="fa fa-cloud-upload"></i>
                        <p>{ format!("Subir maximo {max_files}") }</p>
                        <label for="file-upload" class="upload control">
                            <a class="button is-primary">
                                <span class="icon"><i class="fa fa-upload"></i></span>
                                <span>{"Subir imagen"}</span>
                            </a>
                            <input
                                ref={file_picker}
                                id="file-upload"
                                type="file"
                                accept="image/*"
                                multiple={props.multiple}
                                onclick={on_start_upload}
                            />
                        </label>
                    </div>
                </div>
            </div>
            </div>
            </div>
    }
}

fn process_pictures(
    pictures: &UseReducerDispatcher<Reducer>,
    input: Option<web_sys::FileList>,
    accept: Vec<String>,
) -> Vec<FileReader> {
    let pic = load_files(input, accept);
    let mut readers = Vec::new();
    for p in pic {
        let name = p.name().clone();
        log::debug!("Processing: {name}");
        let mime = p.raw_mime_type().clone();
        let id = Uuid::new_v4();
        pictures.dispatch(FileActions::Add(id, name.clone(), mime.clone()));
        readers.push(load_picture(id, &p, pictures));
    }
    log::debug!("readers {:?}", readers);
    readers
}

fn load_picture(id: Uuid, picture: &File, pictures: &UseReducerDispatcher<Reducer>) -> FileReader {
    let photos_dispatcher = pictures.clone();

    gloo::file::callbacks::read_as_bytes(picture, move |res| match res {
        Ok(contents) => {
            photos_dispatcher.dispatch(FileActions::Loaded(id, contents));
        }
        Err(e) => {
            log::error!("Unable to read files: {:?}", e);
        }
    })
}

fn load_files(files: Option<web_sys::FileList>, accept: Vec<String>) -> Vec<File> {
    let mut result = vec![];
    if let Some(list) = files {
        for i in 0..list.length() {
            if let Some(file) = list.get(i) {
                let gf = gloo::file::File::from(file);
                if is_accepted_file_type(gf.raw_mime_type(), accept.clone()) {
                    result.push(gf);
                }
            }
        }
    }
    result
}

fn validate_list(list: &web_sys::DataTransferItemList) -> Option<i32> {
    if list.length() < 1 {
        log::debug!("Too little files");
        return None;
    }
    let mut num = 0;
    for i in 0..list.length() {
        if let Some(item) = list.get(i) {
            if item.kind().eq("file") {
                num += 1;
            }
        }
    }
    if num > 0 {
        Some(num)
    } else {
        None
    }
}

fn is_accepted_file_type(file_type: String, accepted: Vec<String>) -> bool {
    // contains exact category and extension 
    if accepted.contains(&file_type) { return true }
    // check for category and all extensions
    else {
        let file_category: &str = file_type.split("/").next().unwrap();
        for mime in accepted {
            let mut s = mime.split("/");
            if let Some(category) = s.next() {
                if let Some(extension) = s.next() {
                    if file_category == category && extension == "*" {
                        log::debug!("category {category}");
                        log::debug!("file category {file_category}");
                        log::debug!("extension {extension}");
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

/*
    if file.bytes.is_some() && !file.started_upload {
        let bytes = file.bytes.clone();
        let name = file.name.clone();
        let mime = file.mime.clone();
        let id = file.id.clone();
        pictures_dispatcher.dispatch(FileActions::UploadStarted(id));
        spawn_local(async move {
            let multipart = match bytes {
                None => {
                    return;
                }
                Some(data) => {
                    //pictures_dispatcher.dispatch(FileActions::Loaded(id, data.clone()));
                    log::debug!("starting upload image");
                    let multipart = reqwest::multipart::Form::new();
                    let mut file = reqwest::multipart::Part::bytes(data);
                    file = file.file_name(name.clone());
                    file = match file.mime_str(&mime) {
                        Ok(f) => f,
                        Err(e) => {
                            log::error!("Unable to set mime type: {e}");
                            return;
                        }
                    };
                    multipart.part(name.clone(), file)
                }
            };
            
            let response = request_admin_update_vehicule_picture("85c471d7-2a77-41f7-8a91-6a12cceafe47".to_string(), multipart).await;
            match response {
                Ok(r) => {
                    log::debug!("image uploaded successfully\n {:?}", r);
                }
                Err(e) => {
                    log::error!("image uploaded failed\n {:?}", e);
                }
            }
        });
*/
