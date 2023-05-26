use yew::prelude::*;

use common::models::user::Usuario;

use crate::components::form::{FormField, StaticField};
use crate::components::card::{Card, CardContent};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub user_state: UseStateHandle<Usuario>,
}

#[function_component]
pub fn AdminProfileInfo(props: &Props) -> Html {

    let Props { user_state } = props;
    let user = (*user_state).clone();

    // create picture url
    let picture_url = user.imagen_url("http://127.0.0.1:8000/");

    // Map fields to nice strings
    let numero_empleado = match user.numero_empleado{
        Some(n) => n.to_string(),
        None => "No asignado".to_string(),
    };

    let verificado = match user.verificado {
        true => "si".to_string(),
        false => "no".to_string(),
    };

    let activo = match user.activo {
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
                        <StaticField value={ user.usuario_id.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Nombre" is_horizontal={false}>
                        <StaticField value={ user.nombres.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Apellido" is_horizontal={false}>
                        <StaticField value={ user.apellidos.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Correo Electrónico" is_horizontal={false}>
                        <StaticField value={ user.email.clone() }/>
                    </FormField>

                    <hr/>
                    
                    <FormField label="Numero de empleado" is_horizontal={false}>
                        <StaticField value={ numero_empleado }/>
                    </FormField>
                    
                    <hr/>

                    <FormField label="Departamento" is_horizontal={false}>
                        <StaticField value={ user.departamento.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Rol" is_horizontal={false}>
                        <StaticField value={ user.rol.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Cuenta Verificada" is_horizontal={false}>
                        <StaticField value={ verificado }/>
                    </FormField>

                    <hr/>

                    <FormField label="Activo" is_horizontal={false}>
                        <StaticField value={ activo }/>
                    </FormField>

                    <hr/>

                    <FormField label="Ultima modificacion" is_horizontal={false}>
                        <StaticField value={ user.modificado_en.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Fecha de creacion" is_horizontal={false}>
                        <StaticField value={ user.creado_en.to_string() }/>
                    </FormField>
                </CardContent>
            </Card>
    }
}
