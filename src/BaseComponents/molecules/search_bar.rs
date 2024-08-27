pub use dioxus::prelude::*;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;

use crate::{
    collections::SEARCH,
    main_page::ARROW_LEFT,
    BaseComponents::{
        atoms::button::{Button, FillMode, Roundness},
        string_placements::ContentType,
    },
};

#[component]
pub fn FuzzyFilterer(
    search: ReadOnlySignal<String>,
    default: String,
    childrens: Vec<(String, Element)>,
) -> Element {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
    let search = search();
    let render = childrens
        .into_iter()
        .map(|(name, x)| {
            let score = matcher.fuzzy_match(&name, &search);
            (score, x)
        })
        .filter_map(|(score, x)| {
            if search == default {
                Some((i64::MAX, x))
            } else {
                score.map(|score| (score, x))
            }
        })
        .sorted_by_key(|x| std::cmp::Reverse(x.0))
        .map(|x| x.1);
    rsx! {
        for x in render {
            {x}
        }
    }
}

#[component]
pub fn SearchBar(search: Signal<String>, default: String) -> Element {
    let default = CopyValue::new(default);
    rsx! {
        Button {
            roundness: Roundness::Squircle,
            extended_css_class: "bg-background min-w-fit w-full",
            fill_mode: FillMode::Fit,
            clickable: false,
            string_placements: vec![
                ContentType::custom(rsx!(
                    input {
                        class: "w-full text-hint font-medium text-xl leading-[1.2] capsize",
                        onfocusin: move |_| {
                            if &*search.read() == &default.cloned() {
                                search.set(String::new());
                            }
                        },
                        onfocusout: move |_| {
                            search.set(default.cloned());
                        },
                        oninput: move |x| {
                            search.set(x.value());
                        },
                        value: search(),
                    }
                ))
                .align_left()
                .css("grow min-w-full justify-self-stretch"),
                ContentType::svg(SEARCH).css("shrink-0").align_right(),
            ],
        }
    }
}
