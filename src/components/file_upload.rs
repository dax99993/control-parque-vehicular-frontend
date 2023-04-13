use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement, MouseEvent};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html, Properties, classes};



#[derive(Debug)]
struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
    Delete(String),
}

#[derive(Debug, PartialEq, Properties)]
pub struct UploadFileProps {
    pub accept: Vec<String>,
    pub multiple: bool,
    pub max_files: usize,
    //Callback submit
    //Preview: bool
}

#[derive(Debug)]
pub struct UploadFile {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    full: bool,
}

impl Component for UploadFile {
    type Message = Msg;
    type Properties = UploadFileProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
            full: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Delete(file_name) => {
                let len = self.files.len();
                self.files.retain_mut(|file| file.name != file_name);
                let current_len = self.files.len();

                if self.full && current_len != len {
                    self.full = false;
                }
                
                true
            }
            Msg::Loaded(file_name, file_type, data) => {
                self.full = self.files.len() >= ctx.props().max_files;

                if !self.full {
                    self.files.push(FileDetails {
                        data,
                        file_type,
                        name: file_name.clone(),
                    });
                    self.readers.remove(&file_name);

                }
                
                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();
                    // Ignore files
                    if self.full {
                        //maybe need to return false
                        break; 
                    }
                    // here should ignore non accepted file types
                    if !Self::accepted_file_type(file_type.clone(), ctx.props().accept.clone()) {
                        continue;
                    }

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("upload props: {:?}", ctx.props());
        let accept = ctx.props().accept.join(",");
        let multiple = ctx.props().multiple.clone();
        let max_files = ctx.props().max_files.clone();

        html! {
            <div class="container">
            <div class="columns is-centered has-text-centered">
            <div class="card column is-half">
                <div class={classes!["card-content", if self.files.len() == 0 { "is-hidden" } else {""}]}>
                    { 
                        for self.files.iter().map(|f| {
                            let filename = f.name.clone();
                            let cb = ctx.link().callback(move |event: MouseEvent| {
                                    event.prevent_default();
                                    Msg::Delete(filename.clone())
                            });
                            Self::view_file(f, cb)
                            })
                    }
                </div>
                <div class="card-footer has-background-grey-lighter">
                    <div class="column"
                            ondrop={ctx.link().callback(|event: DragEvent| {
                                event.prevent_default();
                                let files = event.data_transfer().unwrap().files();
                                Self::upload_files(files)
                            })}
                            ondragover={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                            ondragenter={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
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
                                id="file-upload"
                                type="file"
                                {accept}
                                {multiple}
                                onchange={ctx.link().callback(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Self::upload_files(input.files())
                                })}
                            />
                        </label>
                    </div>
                </div>
            </div>
            </div>
            </div>
        }
    }
}

impl UploadFile {

    fn view_file(file: &FileDetails, cb: Callback<MouseEvent>) -> Html {
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
                    if file.file_type.contains("image") {
                        <img src={format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data))} />
                    } else if file.file_type.contains("video") {
                        <video controls={true}>
                            <source src={format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data))} type={file.file_type.clone()}/>
                        </video>
                    }
                    </figure>
                </div>
            </div>
        }
    }


    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }

    fn accepted_file_type(file_type: String, accepted: Vec<String>) -> bool {
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
}
