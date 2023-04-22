

#[derive(Debug,  Clone, PartialEq, Default)]
pub struct MultipartPart {
   pub bytes: Vec<u8>,
   pub name: String,
   pub mime: String,
}


#[derive(Debug,  Clone, PartialEq, Default)]
pub struct MultipartForm {
    pub parts: Vec<MultipartPart>,
}

impl MultipartForm {
    pub fn append(&mut self, part: MultipartPart) {
        self.parts.push(part);
    }

    pub fn into_reqwest_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        for part in self.parts {
            let bytes = part.bytes.clone();
            let name = part.name.clone();
            let mime = part.mime.clone();
            let mut file = reqwest::multipart::Part::bytes(bytes);
            file = file.file_name(name.clone());
            file = match file.mime_str(&mime) {
                Ok(f) => f,
                Err(_) => {
                    continue;
                }
            };
            form = form.part(name.clone(), file)
        }
        return form;
    }
}
