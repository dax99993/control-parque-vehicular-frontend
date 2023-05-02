use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct StaticFieldProps {
    pub value: String,
}

#[function_component]
pub fn StaticField(props: &StaticFieldProps) -> Html {
    let StaticFieldProps{ value } = props;

    html!{
        <div class="control is-clearfix">
            <input type="text" readonly={true} value={value.clone()} class="input is-static"/>
        </div>
    }
}
