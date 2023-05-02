use yew::prelude::*;

use crate::shadow_clone;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children, 
}


#[function_component]
pub fn VehiculeTable(props: &Props) -> Html {
    shadow_clone!(props);

    html!{
    <div class="b-table has-pagination">
        <div class="table-wrapper has-mobile-cards table-container">
            <table class="table is-narrow is-striped is-hoverable is-fullwidth">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Marca"}</th>
                        <th>{"Modelo"}</th>
                        <th>{"Año"}</th>
                        <th>{"Nombre economico"}</th>
                        <th>{"Numero de tarjeta"}</th>
                        <th>{"Numero de placa"}</th>
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
