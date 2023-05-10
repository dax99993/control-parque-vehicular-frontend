use yew::prelude::*;

use crate::shadow_clone;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children, 
}


#[function_component]
pub fn UsersTable(props: &Props) -> Html {
    shadow_clone!(props);

    html!{
    <div class="b-table has-pagination">
        <div class="table-wrapper has-mobile-cards table-container">
            <table class="table is-narrow is-striped is-hoverable is-fullwidth">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Nombres"}</th>
                        <th>{"Apellidos"}</th>
                        <th>{"Departamento"}</th>
                        <th>{"Numero de empleado"}</th>
                        <th>{"Correo Electronico"}</th>
                        <th>{"Estado"}</th>
                        <th>{"Activo"}</th>
                        <th>{"Ultima modificacion"}</th>
                        <th>{"Fecha de creacion"}</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                {
                    props.children
                }
                </tbody>
            </table>
        </div>
    </div>
    }
}
