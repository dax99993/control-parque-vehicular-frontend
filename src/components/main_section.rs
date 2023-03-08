use yew::prelude::*;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MainSectionProps {
    pub children: Children,
}


#[function_component]
pub fn MainSection(props: &MainSectionProps) -> Html {

    log::debug!("{}", props.children.len());

    let props = props.clone();
    let mut iter = props.children.iter();
    let sidebar = iter.next();
    let rightpart = iter.next();
    html!{
        <ybc::Section>
            <ybc::Columns>
                <ybc::Column 
                    classes={classes!(
                            "is-4-tablet",
                            "is-3-desktop",
                            "is-2-widescreen",
                            )}>
                    { sidebar }
                </ybc::Column>
            
                <ybc::Column>
                    { rightpart }
                </ybc::Column>
            </ybc::Columns>
        </ybc::Section>
    }
}
