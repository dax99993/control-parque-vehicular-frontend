use yew::prelude::*;

use common::models::vehicule::Vehiculo;

use crate::components::form::FormField;
use crate::components::card::{Card, CardContent};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeShowProps {
    pub estado_vehiculo: UseStateHandle<Vehiculo>,
}

#[function_component]
pub fn EditVehiculeShow(props: &EditVehiculeShowProps) -> Html {
    let EditVehiculeShowProps { estado_vehiculo } = props;
    let vehiculo = (*estado_vehiculo).clone();

    // crear imagen url
    let imagen_url = vehiculo.imagen_url("http://127.0.0.1:8000/");

    let active = vehiculo.activo_a_palabra();

    html!{
            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Vehiculo"}
            >
                <CardContent>
                    <div class="is-user-avatar image has-max-width is-aligned-center">
                        <img src={ imagen_url } />
                    </div>
                    <hr/>

                    <FormField label="Id">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.vehiculo_id.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Marca">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.marca.clone() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Modelo">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.modelo.clone() } class="input is-static"/>
                        </div>
                    </FormField>
                    
                    <FormField label="Año">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.año.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Placa">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.numero_placa.clone() } class="input is-static"/>
                        </div>
                    </FormField>
                    
                    <FormField label="Numero de tarjeta">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.numero_tarjeta.clone() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Nombre economico">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.nombre_economico.clone() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Estado">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.estado.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Activo">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ active } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Ultima modificacion">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.modificado_en.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Fecha de creacion">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehiculo.creado_en.to_string() } class="input is-static"/>
                        </div>
                    </FormField>
                </CardContent>
            </Card>
    }
}
