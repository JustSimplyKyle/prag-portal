use dioxus::prelude::*;
#[component]
pub fn Center(
    children: Element,
    #[props(default = 1./3.)] percentage_center_bias: f64,
    #[props(extends = GlobalAttributes, extends = div)] mut attributes: Vec<Attribute>,
) -> Element {
    let side_bias = (100. - percentage_center_bias) / 2.;
    rsx! {
        div {
            flex_basis: "{side_bias}%",
        }
        div {
            flex_basis: "{percentage_center_bias}%",
            ..attributes,
            {children}
        }
        div {
            flex_basis: "{side_bias}%",
        }
    }
}
