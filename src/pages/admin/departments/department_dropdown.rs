use std::ops::Deref;

use yew::prelude::*;
use yew_hooks::prelude::*;

//use common::models::department::Departamento;

use crate::components::dropdown::DropDown;

use crate::services::admin::request_get_departments;

#[function_component]
pub fn DepartmentDropDown() -> Html {

    let departamentos = use_state(|| vec![]);
    //let departamentos = use_state(|| Vec::<Departamento>::default());
    let dep_seleccionado_nombre = use_state(|| None::<String>);
    //let dep_seleccionado_id= use_state(|| None::<i32>);


    // request department
    let request_departamentos = {
        use_async(async {
            request_get_departments().await
        })  
    };

    // Fetch cuando se renderiza por primera vez
    {
        let request_departamentos = request_departamentos.clone();
        use_effect_with_deps(move |_| {
           request_departamentos.run(); 
        },
        ());
    }

    // Almacenar departamentos
    {
        let departamentos = departamentos.clone();
        use_effect_with_deps(move |request| {
            if let Some(response) = &request.data {
                if let Some(d) = response.data.clone() {
                    departamentos.set(d);
                }
            }
        }, request_departamentos.clone())
    }

    // Crear vector de nombres de departamento
    let departamento_nombres = departamentos.deref().iter().map(|d| d.nombre.clone()).collect::<Vec<String>>();

    //
    let dep_seleccionado_id = {
        if let Some(s) = dep_seleccionado_nombre.deref().clone() {
            departamentos.deref().iter()
                    .filter(|d| d.nombre.eq(&s) ).map(|d| d.id)
                    .next()
        } else {
            None
        }
    };

    log::debug!("id {:?} departamento {:?}", dep_seleccionado_id, dep_seleccionado_nombre);

    html!{

        <DropDown dropdown_button_label={"Departamento"} selected_state={dep_seleccionado_nombre.clone()} dropdown_items_labels={departamento_nombres}/>
    }
}
