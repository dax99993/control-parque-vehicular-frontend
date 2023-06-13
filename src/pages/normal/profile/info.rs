use yew::prelude::*;
use yew::platform::spawn_local;

use common::models::user::Usuario;

use crate::components::form::{FormField, StaticField};
use crate::components::card::{Card, CardContent};


use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;



#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub usuario_state: UseStateHandle<Usuario>,
}

#[function_component]
pub fn ProfileInfo(props: &Props) -> Html {
    //Props
    let Props { usuario_state } = props;
    let usuario = (*usuario_state).clone();

    //States
    let imagen = use_state(|| vec![]);

    // Hooks
    // Maybe use a future to handle image fail
    {
        let imagen = imagen.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let response = crate::services::normal::request_imagen_perfil().await;
                match response {
                    Ok(bytes) => {
                        imagen.set(bytes.clone());
                    }
                    Err(e) => {
                        log::error!("peticion de imagen fallo {:?}", e);
                    }
                }
            });
        }, usuario.clone())
    }


    //Variables
    let numero_empleado = match usuario.numero_empleado{
        Some(n) => n.to_string(),
        None => "No asignado".to_string(),
    };

    let verificado = match usuario.verificado {
        true => "Si".to_string(),
        false => "No".to_string(),
    };


    html!{
            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Perfil Usuario"}
            >
                <CardContent>
                    <div class="is-user-avatar image has-max-width is-aligned-center">
                        if !imagen.deref().is_empty() {
                            <img src={ format!("data:image/jpeg;base64,{}", STANDARD.encode(&imagen.deref())) } />
                        }
                    </div>

                    <hr/>

                    <FormField label="Id" is_horizontal={false}>
                        <StaticField value={ usuario.usuario_id.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Nombre" is_horizontal={false}>
                        <StaticField value={ usuario.nombres.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Apellidos" is_horizontal={false}>
                        <StaticField value={ usuario.apellidos.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Correo Electrónico" is_horizontal={false}>
                        <StaticField value={ usuario.email.clone() }/>
                    </FormField>

                    <hr/>
                    
                    <FormField label="Numero de empleado" is_horizontal={false}>
                        <StaticField value={ numero_empleado }/>
                    </FormField>
                    
                    <hr/>

                    <FormField label="Departamento" is_horizontal={false}>
                        <StaticField value={ usuario.departamento.clone() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Rol" is_horizontal={false}>
                        <StaticField value={ usuario.rol.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Cuenta Verificada" is_horizontal={false}>
                        <StaticField value={ verificado }/>
                    </FormField>

                    <hr/>

                    <FormField label="Activo" is_horizontal={false}>
                        <StaticField value={ usuario.activo_a_palabra() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Ultima modificacion" is_horizontal={false}>
                        <StaticField value={ usuario.modificado_en.to_string() }/>
                    </FormField>

                    <hr/>

                    <FormField label="Fecha de creacion" is_horizontal={false}>
                        <StaticField value={ usuario.creado_en.to_string() }/>
                    </FormField>
                </CardContent>
            </Card>
    }
}
