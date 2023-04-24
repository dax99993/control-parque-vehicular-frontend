use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use validator::{Validate, ValidationError};

use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Vehicule {
    pub vehicule_id: Uuid,
    pub branch: String,
    pub model: String,
    pub year: i16,
    pub number_plate: String,
    pub short_name: String,
    pub number_card: String,
    pub status: String,
    pub active: bool,
    pub picture: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


impl Vehicule {
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn is_available(&self) -> bool {
        self.status == "available".to_string()
    }
    pub fn is_occupied(&self) -> bool {
        self.status == "occupied".to_string()
    }
    pub fn is_maintenance(&self) -> bool {
        self.status == "maintenance".to_string()
    }

    pub fn update(mut self, update: UpdateVehicule) -> Self {
        self.branch = update.branch.unwrap_or_else(|| self.branch);
        self.model = update.model.unwrap_or_else(|| self.model);
        self.year = update.year.unwrap_or_else(|| self.year);
        self.number_plate= update.number_plate.unwrap_or_else(|| self.number_plate);
        self.short_name= update.short_name.unwrap_or_else(|| self.short_name);
        self.number_card= update.number_card.unwrap_or_else(|| self.number_card);
        self.status= update.status.unwrap_or_else(|| self.status);
        self.active= update.active.unwrap_or_else(|| self.active);
        //self.picture= update.picture.unwrap_or_else(|| self.picture);

        self
    }

    pub fn status_to_spanish(&self) -> String {
        if self.is_occupied() {
            return "ocupado".to_string();
        }
        if self.is_available() {
            return "disponible".to_string();
        }
        if self.is_maintenance() {
            return "mantenimiento".to_string();
        }
        else { return "desconocido".to_string() };
    }

    pub fn active_to_spanish(&self) -> String {
        if self.active {
            return "si".to_string();
        } else { 
            return "no".to_string()
        };
    }

    pub fn get_picture_url(&self, base_url: &str) -> String {
        format!("{base_url}api/images?filename={}", self.picture)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct NewVehicule {
    #[validate(
        length(min = 1, message = "Marca requerida"),
    )]
    pub branch: String,
    #[validate(
        length(min = 1, message = "Modelo requerido"),
    )]
    pub model: String,
    #[validate(
        range(min = 0, message = "Año requerido"),
    )]
    pub year: i16,
    #[validate(
        length(min = 1, message = "Placa requerida"),
    )]
    pub number_plate: String,
    #[validate(
        length(min = 1, message = "Nombre economico requerido"),
    )]
    pub short_name: String,
    #[validate(
        length(min = 1, message = "Numero de tarjeta requerido"),
    )]
    pub number_card: String,
    //pub status: String,
    //pub active: bool,
    //pub picture: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct UpdateVehicule {
    #[validate(
        length(min = 1, message = "Marca requerida"),
    )]
    pub branch: Option<String>,
    #[validate(
        length(min = 1, message = "Modelo requerido"),
    )]
    pub model: Option<String>,
    #[validate(
        range(min = 0, message = "Año requerido"),
    )]
    pub year: Option<i16>,
    #[validate(
        length(min = 1, message = "Nombre economico requerido"),
    )]
    pub number_plate: Option<String>,
    #[validate(
        length(min = 1, message = "Nombre economico requerido"),
    )]
    pub short_name: Option<String>,
    #[validate(
        length(min = 1, message = "Numero de tarjeta requerido"),
    )]
    pub number_card: Option<String>,
    #[validate(
        custom(function = "validate_status")
    )]
    pub status: Option<String>,
    pub active: Option<bool>,
    //pub picture: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FilteredVehicule {
    pub vehicule_id: Uuid,
    pub branch: String,
    pub model: String,
    pub year: i16,
    pub number_plate: String,
    pub short_name: String,
    pub number_card: String,
    //pub status: String,
    //pub active: bool,
    pub picture: String,
    //pub created_at: NaiveDateTime,
    //pub updated_at: NaiveDateTime,
}

impl FilteredVehicule {
    pub fn from(veh: Vehicule) -> Self {
        Self { 
            vehicule_id: veh.vehicule_id,
            branch: veh.branch,
            model: veh.model,
            year: veh.year,
            number_plate: veh.number_plate,
            short_name: veh.short_name,
            number_card: veh.number_card,
            picture: veh.picture,
        }
    }
}


fn validate_status<'a, T>(value: T) -> Result<(), ValidationError>
where 
    T: Into<Cow<'a, str>>
{
    match value.into() {
        std::borrow::Cow::Borrowed("available") |
        std::borrow::Cow::Borrowed("maintenance") |
        std::borrow::Cow::Borrowed("occupied") => { return Ok(()) },
        _ => {
            let error = ValidationError::new("Estado invalido");
            return Err(error);
        }
    }
}
