use dioxus::prelude::*;
use dioxus_logger::tracing::warn;

#[component]
pub fn RenderTranslatedMarkdown(html: String) -> Element {
    if html.contains("Create") {
        println!("{}", &html);
    }
    rsx! {
        div {
            class: "px-[30px] py-[10px] [&_*]:font-english bg-deep-background
                        [&_h4]:text-[16px] [&_h4]:font-semibold [&_h4]:my-[12.5px]
                        [&_h3]:text-[18px] [&_h3]:font-semibold [&_h3]:my-[12.5px]
                        [&_h2]:text-[20px] [&_h2]:font-semibold [&_h2]:my-[12.5px]
                        [&_h1]:text-[25px] [&_h1]:font-semibold [&_h1]:my-[12.5px]

                        [&_li]:text-white [&_li]:font-medium [&_li]:text-[15px] 
                        [&_li_p]:text-white
                        [&_ul]:list-inside [&_ul]:list-disc [&_ul]:my-[12.5px] 

                        [&_strong]:text-white
                        [&_a]:underline [&_a]:text-light-blue
                        [&_p]:text-secondary-text [&_p]:font-medium [&_p]:text-[15px]

                        [&_image]:my-[12.5px]
                        ",
            dangerous_inner_html: html,
        }
    }
}
