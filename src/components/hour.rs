use chrono::NaiveTime;
use yew::prelude::*;

//use super::dropdown::DropDown;

#[derive(PartialEq, Properties)]
pub struct TimePickerProps {
    pub time: UseStateHandle<Option<NaiveTime>>
}

#[function_component]
pub fn TimePicker(props: &TimePickerProps) -> Html {
    /*
    let TimePickerProps { time } = props;
    let hour = use_state(|| None::<String>);
    let min = use_state(|| None::<String>);

    if let Some(h) = &*hour {
        if let Some(m) = &*min {
            let hora = h.parse::<u32>().unwrap();
            let min = m.parse::<u32>().unwrap();
            time.set(NaiveTime::from_hms_opt(hora, min, 0));
        }
    }

    html!{
        <>
            <DropDown dropdown_button_label={"HH"}
                dropdown_items_labels={vec!["00".into(), "01".into(), "02".into()]}
                selected_state={hour.clone()}
            />
            <DropDown dropdown_button_label={"MM"}
                dropdown_items_labels={vec!["00".into(), "10".into(), "20".into(), "30".into(), "40".into(), "50".into()]}
                selected_state={min.clone()}
            />
        </>
    }
    */

    let TimePickerProps { time } = props;

    let onchange = {
        let time = time.clone();
        Callback::from(move |event: Event| {
            let value = event.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            //let (hour, min) = value.split(":").collect(); 
            //let hora = hour.parse::<u32>().unwrap();
            //let min = min.parse::<u32>().unwrap();
            //time.set(NaiveTime::from_hms_opt(hora, min, 0));
            let t = value.split(":").into_iter().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            time.set(NaiveTime::from_hms_opt(*t.get(0).unwrap(), *t.get(1).unwrap(), 0));
        })
    };

    html!{
        <input type="time" {onchange} />
    }
}
