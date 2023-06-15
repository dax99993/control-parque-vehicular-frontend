use common::models::vehicule::Vehiculo;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::shadow_clone;
use crate::hooks::user_context::use_user_context;
use crate::layout::main_section::MainSection;
use crate::components::card::{Card, CardContent};
use crate::components::calendar::{CalendarLang, CalendarColor, Calendar};


#[function_component]
pub fn NormalRequestVehiculeView() -> Html {
    // HTML
    {
        html!{
        <MainSection route="Usuario" subroute="Peticiones" title="Solicitar Vehiculo">
        <Card header_icon_left={ "fa-solid fa-ballot" } header_title={ "Seleccionar horario" }>
            <CardContent>
                <VehiculeSelect/>
            </CardContent>
        </Card>
        </MainSection>
        }
    }
}

use chrono::{Weekday, Utc, NaiveDate, NaiveTime, NaiveDateTime};
use chrono_utilities::naive::DateTransitions;
use crate::components::hour::TimePicker;

#[derive(PartialEq, Properties)]
pub struct DateSelectorProps {
    pub start_date: UseStateHandle<Option<NaiveDateTime>>,
    pub end_date: UseStateHandle<Option<NaiveDateTime>>,
}

use crate::components::form::{Form, FormField};

#[function_component]
fn DateSelector(props: &DateSelectorProps) -> Html {
    let date = use_state(|| None::<NaiveDate>);
    let tiempo_inicial = use_state(|| None::<NaiveTime>);
    let tiempo_final= use_state(|| None::<NaiveTime>);

    if let Some(date) = *date {
        if let Some(tiempo) = &*tiempo_inicial {
                let datetime = NaiveDateTime::new(date.clone(), tiempo.clone());
                props.start_date.set(Some(datetime));
        }
        if let Some(tiempo) = &*tiempo_final {
                let datetime = NaiveDateTime::new(date.clone(), tiempo.clone());
                props.end_date.set(Some(datetime));
        }
    }

    html!{
        <>
        <Form>
            <FormField label="Fecha">
                <Calendar {date} color={CalendarColor::Warning} lang={CalendarLang::Spanish} 
                    disable_weekdays={vec![Weekday::Sun, Weekday::Sat]}
                    min_date={Some(Utc::now().with_timezone(&chrono_tz::Mexico::General).date_naive())}
                    max_date={Utc::now().date_naive().end_of_month()}
                />
            </FormField>
            <hr/>
            <FormField label="Hora de salida">
                <TimePicker time={tiempo_inicial}/>
            </FormField>
            <hr/>
            <FormField label="Hora de llegada">
                <TimePicker time={tiempo_final}/>
            </FormField>
        </Form>
        </>
    }
}

use super::table::{Table, TableReducer, vehicule_to_table_row};
use crate::components::modal::Modal;

#[function_component]
pub fn VehiculeSelect() -> Html {
    let start_date = use_state(|| None::<NaiveDateTime>);
    let end_date = use_state(|| None::<NaiveDateTime>);
    let vehicules = use_state(|| vec![]);
    let reducer = use_reducer(TableReducer::default);


    {
        let vehicules = vehicules.clone();
        use_effect_with_deps(move |(start, end)| {
            if let Some(start) = &**start {
                if let Some(end) = &**end {
                    log::debug!("start date {:?}", start);
                    log::debug!("end date {:?}", end);
                    log::debug!("Make request of vehicules");
                    let vehicules = vehicules.clone();
                    spawn_local(async move {
                        //let response = crate::services::normal::request_get_vehiculos(pagina, vehiculos_por_pagina, filter, filter_value)
                        let response = crate::services::normal::request_get_vehiculos(0, 5, None, None).await;
                        match response {
                            Ok(api_response) => {
                                vehicules.set(api_response.data.unwrap().clone());
                            },
                            Err(api_err) => {
                                log::error!("api request error");
                            }
                        }
                    })
                }
            }

        }, (start_date.clone(), end_date.clone()))
    }
    
    html!{
        <>
        <DateSelector start_date={start_date.clone()} end_date={end_date.clone()} />
        <hr/>
        <Card header_icon_left={ "fa-solid fa-ballot" } header_title={ "Seleccionar Vehiculo" }>
            <CardContent>
            <Table>
            {vehicule_to_table_row((*vehicules).clone(), reducer.dispatcher())}
        </Table>
            <Modal
            id="vehicule-picture-modal"
            body={html!{{"picture!"}}}
        />
            </CardContent>
        </Card>
        </>
    }
}
