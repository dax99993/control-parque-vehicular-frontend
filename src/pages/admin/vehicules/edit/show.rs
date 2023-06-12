use yew::prelude::*;
use yew::platform::spawn_local;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;

use common::models::vehicule::Vehiculo;

use crate::components::form::{FormField, StaticField};
use crate::components::card::{Card, CardContent};

use crate::services::admin::request_vehicule_picture;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeShowProps {
    pub estado_vehiculo: UseStateHandle<Vehiculo>,
}

#[function_component]
pub fn EditVehiculeShow(props: &EditVehiculeShowProps) -> Html {
    // Props
    let EditVehiculeShowProps { estado_vehiculo } = props;
    let vehiculo = (*estado_vehiculo).clone();

    //States
    let imagen = use_state(|| vec![]);

    // Hooks
    {
        let imagen = imagen.clone();
        use_effect_with_deps(move |vehiculo| {
            let imagen_filename = vehiculo.imagen.clone();
            spawn_local(async move {
                let response = request_vehicule_picture(imagen_filename).await;
                log::debug!("ejecutando peticion de imagen");
                match response {
                    Ok(bytes) => {
                        imagen.set(bytes.clone());
                    }
                    Err(e) => {
                        log::error!("peticion de imagen fallo {:?}", e);
                    }
                }
            });
        }, vehiculo.clone())
    }


    //Variables
    let activo = vehiculo.activo_a_palabra();


    //HTML
    html!{
            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Vehiculo"}
            >
                <CardContent>
                    <div class="is-user-avatar image has-max-width is-aligned-center">
                        if !imagen.deref().is_empty() {
                            <img src={ format!("data:image/jpeg;base64,{}", STANDARD.encode(&imagen.deref())) } />
                        }
                    </div>

                    <hr/>

                    <FormField label="Id" is_horizontal={false}>
                        <StaticField value={ vehiculo.vehiculo_id.to_string() }/>
                    </FormField>

                    <FormField label="Marca" is_horizontal={false}>
                        <StaticField value={ vehiculo.marca.clone() }/>
                    </FormField>

                    <FormField label="Modelo" is_horizontal={false}>
                        <StaticField value={ vehiculo.modelo.clone() }/>
                    </FormField>
                    
                    <FormField label="Año" is_horizontal={false}>
                        <StaticField value={ vehiculo.año.to_string() }/>
                    </FormField>

                    <FormField label="Placa" is_horizontal={false}>
                        <StaticField value={ vehiculo.numero_placa.clone() }/>
                    </FormField>
                    
                    <FormField label="Numero de tarjeta" is_horizontal={false}>
                        <StaticField value={ vehiculo.numero_tarjeta.clone() }/>
                    </FormField>

                    <FormField label="Nombre economico" is_horizontal={false}>
                        <StaticField value={ vehiculo.nombre_economico.clone() }/>
                    </FormField>

                    <FormField label="Estado" is_horizontal={false}>
                        <StaticField value={ vehiculo.estado.to_string() }/>
                    </FormField>

                    <FormField label="Activo" is_horizontal={false}>
                        <StaticField value={ activo }/>
                    </FormField>

                    <FormField label="Ultima modificacion" is_horizontal={false}>
                        <StaticField value={ vehiculo.modificado_en.to_string() }/>
                    </FormField>

                    <FormField label="Fecha de creacion" is_horizontal={false}>
                        <StaticField value={ vehiculo.creado_en.to_string() }/>
                    </FormField>
                </CardContent>
            </Card>
    }
}
