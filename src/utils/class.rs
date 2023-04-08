use web_sys::Element;

pub fn toggle_class(e: &Element, class: &str) {
    let classes = e.class_name();
    let mut classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    if classes.contains(&class) {
        classes.retain(|&c| c != class);
    } else {
        classes.push(class);
    };
    e.set_class_name(&classes.join(" "));
}

pub fn add_class(e: &Element, class: &str) {
    let classes = e.class_name();
    let mut classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    if !classes.contains(&class) {
        classes.push(class);
    }
    e.set_class_name(&classes.join(" "));
}

pub fn remove_class(e: &Element, class: &str) {
    let classes = e.class_name();
    let mut classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    if classes.contains(&class) {
        classes.retain(|&c| c != class);
    }
    e.set_class_name(&classes.join(" "));
}

pub fn has_class(e: &Element, class: &str) -> bool {
    let classes = e.class_name();
    let classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    return classes.contains(&class);
}
