use yew::prelude::*;

use common::models::user::Usuario;

use crate::shadow_clone;
use crate::features::users::reducer::{UsersAction, UsersReducer};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
   pub user: Usuario, 
   pub dispatcher: UseReducerDispatcher<UsersReducer>,
}

#[function_component]
pub fn UsersTableRow(props: &Props) -> Html {
    let Props { user , dispatcher } = props;

    //TODO request user picture
    // by constructing a global URL_BASE
    let picture_url = user.imagen_url("http://127.0.0.1:8000/");

    
    let click_show = {
        shadow_clone![user, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = user.usuario_id.clone();
            dispatcher.dispatch(UsersAction::ShowUserPicture(id));
        })
    };
    
    let click_delete = {
        shadow_clone![user, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = user.usuario_id.clone();
            dispatcher.dispatch(UsersAction::DeleteUser(id));
        })
    };
    

    let click_edit = {
        shadow_clone![user, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = user.usuario_id.clone();
            dispatcher.dispatch(UsersAction::UpdateInfo(id));
        })
    };


    // Should make a request to get department from db 
    // and get the actual department name instead of row id
    let departamento = {
        match user.departamento {
            Some(d) => d.to_string(),
            None => "Sin Asignar".to_string(),
        }
    };

    let numero_empleado = {
        match user.numero_empleado{
            Some(d) => d.to_string(),
            None => "Sin Asignar".to_string(),
        }
    };

    html!{
        <tr>
        <td class="is-image-cell">
            <figure class="image is-16by9">
                <img src={picture_url} />
            </figure>
        </td>

        <td data-label="Nombres">{&user.nombres}</td>
        <td data-label="Apellidos">{&user.apellidos}</td>
        <td data-label="Departamento">{departamento}</td>
        <td data-label="Numero de empleado">{numero_empleado}</td>
        <td data-label="Correo Electronico">{&user.email}</td>
        <td data-label="Estado">{ &user.rol.to_string() }</td>
        <td class="has-text-centered" data-label="Activo">{ user.activo_a_palabra() }</td>
        <td data-label="Ultima modificacion">{&user.modificado_en}</td>
        <td data-label="Fecha de creacion">{&user.creado_en}</td>


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
