use yew::prelude::*;
use common::models::vehicule::Vehicule;

use crate::components::form::form::FormField;
use crate::components::card::{Card, CardContent};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeShowProps {
    pub vehicule: UseStateHandle<Vehicule>,
}

#[function_component]
pub fn EditVehiculeShow(props: &EditVehiculeShowProps) -> Html {
    let EditVehiculeShowProps { vehicule: vehicule_state } = props;
    let vehicule = (*vehicule_state).clone();

    // create picture url
    let image = use_state(|| html!{});

    {
        let image = image.clone();
        use_effect_with_deps(move |vehicule| {
            let picture_url = vehicule.get_picture_url("http://127.0.0.1:8000/");
            let img = html!{ <img src={ picture_url } />};
            image.set(img);
        }, 
        vehicule.clone()
        );
    }

    let active = match vehicule.active {
        true => "si".to_string(),
        false => "no".to_string(),
    };

    html!{
            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Vehiculo"}
            >
                <CardContent>
                    <div class="is-user-avatar image has-max-width is-aligned-center">
                        {(*image).clone()}
                    </div>
                    <hr/>

                    <FormField label="Id">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.vehicule_id.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Marca">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.branch.clone() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Modelo">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.model.clone() } class="input is-static"/>
                        </div>
                    </FormField>
                    
                    <FormField label="Año">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.year.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Placa">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.number_plate.clone() } class="input is-static"/>
                        </div>
                    </FormField>
                    
                    <FormField label="Numero de tarjeta">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.number_card.clone() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Nombre economico">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.short_name.clone() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Estado">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.status_to_spanish() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Activo">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ active } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Ultima modificacion">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.updated_at.to_string() } class="input is-static"/>
                        </div>
                    </FormField>

                    <FormField label="Fecha de creacion">
                        <div class="control is-clearfix">
                            <input type="text" readonly={true} value={ vehicule.created_at.to_string() } class="input is-static"/>
                        </div>
                    </FormField>
                </CardContent>
            </Card>
    }
}
