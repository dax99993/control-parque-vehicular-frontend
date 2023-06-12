use yew::prelude::*;
use yew::platform::spawn_local;

use common::models::user::Usuario;

use crate::shadow_clone;
use super::reducer::{UsersAction, UsersReducer};

use crate::services::admin::request_imagen_usuario;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
   pub usuario: Usuario, 
   pub dispatcher: UseReducerDispatcher<UsersReducer>,
}

#[function_component]
pub fn UsersTableRow(props: &Props) -> Html {
    let Props { usuario , dispatcher } = props;

    //States
    let imagen = use_state(|| vec![]);

    // Hooks
    {
        let imagen = imagen.clone();
        use_effect_with_deps(move |usuario| {
            let imagen_filename = usuario.imagen.clone();
            spawn_local(async move {
                let response = request_imagen_usuario(imagen_filename).await;
                log::debug!("ejecutando peticion de imagen");
                match response {
                    Ok(bytes) => {
                        imagen.set(bytes.clone());
                    }
                    Err(_) => {
                        log::error!("peticion de imagen fallo");
                    }
                }
            });
        }, usuario.clone())
    }

    
    //Callbacks
    let click_show = {
        shadow_clone![usuario, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = usuario.usuario_id.clone();
            dispatcher.dispatch(UsersAction::ShowUserPicture(id));
        })
    };
    
    let click_delete = {
        shadow_clone![usuario, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = usuario.usuario_id.clone();
            dispatcher.dispatch(UsersAction::DeleteUser(id));
        })
    };
    

    let click_edit = {
        shadow_clone![usuario, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = usuario.usuario_id.clone();
            dispatcher.dispatch(UsersAction::UpdateInfo(id));
        })
    };


    // Variables
    let numero_empleado = {
        match usuario.numero_empleado{
            Some(d) => d.to_string(),
            None => "Sin Asignar".to_string(),
        }
    };

    let email = usuario.email.clone();
    //let email = usuario.email.replace("@", "\u{200B}@");


    //HTML
    html!{
        <tr>
        <td class="is-image-cell">
            <figure class="image">
                if !imagen.deref().is_empty() {
                    <img src={ format!("data:image/jpeg;base64,{}", STANDARD.encode(&imagen.deref())) } onclick={click_show.clone()}/>
                }
            </figure>
        </td>

        <td data-label="Nombres">{&usuario.nombres}</td>
        <td data-label="Apellidos">{&usuario.apellidos}</td>
        <td data-label="Departamento">{&usuario.departamento}</td>
        <td data-label="Numero de empleado">{numero_empleado}</td>
        <td data-label="Correo Electronico">{&email}</td>
        <td data-label="Estado">{ &usuario.rol.to_string() }</td>
        <td class="has-text-centered" data-label="Activo">{ usuario.activo_a_palabra() }</td>
        <td data-label="Ultima modificacion">{&usuario.modificado_en}</td>
        <td data-label="Fecha de creacion">{&usuario.creado_en}</td>


        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small is-primary" type="button" onclick={click_show}>
                    <span class="icon"><i class="fa-solid fa-eye"></i></span>
                    <span>{"Imagen"}</span>
                </button>

                <button class="button is-info is-small" type="button" onclick={click_edit}>
                    <span class="icon"><i class="fa-solid fa-pen"></i></span>
                    <span>{"Editar"}</span>
                </button>

                <button class="button is-danger is-small" type="button" onclick={click_delete}>
                    <span class="icon"><i class="fa-solid fa-trash-can"></i></span>
                    <span>{"Borrar"}</span>
                </button>
            </div>
        </td>

        </tr>
    }
}
