use web_sys::Element;


pub fn toggle_class(e: Element, class: &str) {
    let e_classes = e.class_name();
    let mut classes: Vec<&str> = e_classes
        .split_whitespace()
        .collect();
    if classes.contains(&class) {
        classes.retain(|&c| c != class);
    } else {
        classes.push(class);
    };
    e.set_class_name(&classes.join(" "));
}
