use yew::prelude::*;

use common::models::user::User;

use crate::components::form::{FormField, StaticField};
use crate::components::card::{Card, CardContent};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub user_state: UseStateHandle<User>,
}

#[function_component]
pub fn AdminProfileInfo(props: &Props) -> Html {

    let Props { user_state } = props;
    let user = (*user_state).clone();

    // create picture url
    let picture_url = user.get_picture_url("http://127.0.0.1:8000/");

    // Map fields to nice strings
    let employee_number = match user.employee_number{
        Some(n) => n.to_string(),
        None => "No asignado".to_string(),
    };

    let department = match user.department {
        Some(n) => n.to_string(),
        None => "No asignado".to_string(),
    };

    let verified = match user.verified{
        true => "si".to_string(),
        false => "no".to_string(),
    };

    let active = match user.active {
        true => "si".to_string(),
        false => "no".to_string(),
    };

    html!{
            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Perfil Usuario"}
            >
                <CardContent>
                    <div class="is-user-avatar image has-max-width is-aligned-center">
                        <img src={ picture_url } />
                    </div>

                    <hr/>

                    <FormField label="Id" is_horizontal={false}>
                        <StaticField value={ user.user_id.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Nombre" is_horizontal={false}>
                        <StaticField value={ user.first_name.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Apellido" is_horizontal={false}>
                        <StaticField value={ user.last_name.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Correo Electrónico" is_horizontal={false}>
                        <StaticField value={ user.email.clone() }/>
                    </FormField>

                    <hr/>
                    
                    <FormField label="Numero de empleado" is_horizontal={false}>
                        <StaticField value={ employee_number }/>
                    </FormField>
                    
                    <hr/>

                    <FormField label="Departamento" is_horizontal={false}>
                        <StaticField value={ department }/>
                    </FormField>

                    <hr/>

                    <FormField label="Rol" is_horizontal={false}>
                        <StaticField value={ user.role.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Cuenta Verificada" is_horizontal={false}>
                        <StaticField value={ verified }/>
                    </FormField>

                    <hr/>

                    <FormField label="Activo" is_horizontal={false}>
                        <StaticField value={ active }/>
                    </FormField>

                    <hr/>

                    <FormField label="Ultima modificacion" is_horizontal={false}>
                        <StaticField value={ user.updated_at.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Fecha de creacion" is_horizontal={false}>
                        <StaticField value={ user.created_at.to_string() }/>
                    </FormField>
                </CardContent>
            </Card>
    }
}
