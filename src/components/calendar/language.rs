
#[derive(PartialEq, Default)]
pub enum CalendarLang {
    #[default]
    English,
    Spanish,
}

impl CalendarLang {
    pub fn months(&self) -> Vec<&str> {
        match self {
            Self::English => {
                vec!["January", "February", "March", "April", "May", "June",
                "July", "August", "September", "Octuber", "November", "December"]
            },
            Self::Spanish => {
                vec!["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio",
                "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"]
            }
        }
    }

    pub fn months_short(&self) -> Vec<&str> {
        match self {
            Self::English => {
                vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
            },
            Self::Spanish => {
                vec!["Ene", "Feb", "Mar", "Abr", "May", "Jun",
                "Jul", "Ago", "Sep", "Oct", "Nov", "Dic"]
            }
        }
    }

    pub fn weekdays(&self) -> Vec<&str> {
        match self {
            Self::English => {
                vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]
            },
            Self::Spanish => {
                vec!["Domingo", "Lunes", "Martes", "Miercoles", "Jueves", "Viernes", "Sabado"]
            }
        }
    }

    pub fn weekdays_short(&self) -> Vec<&str> {
        match self {
            Self::English => {
                vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
            },
            Self::Spanish => {
                vec!["Dom", "Lun", "Mar", "Mie", "Jue", "Vie", "Sab"]
            }
        }
    }
}
