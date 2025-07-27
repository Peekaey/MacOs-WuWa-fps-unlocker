use dioxus::prelude::*;
use crate::helpers::get_local_storage_filepath;

const GITHUB_SVG: Asset = asset!("/assets/github.svg");

#[component]
pub fn Menu() -> Element {
    rsx! {
        div { id: "menu",
            div { id: "menu-content",
                div { id: "menu-content-title",
                    p { "This application is a simple FPS unlocker for Wuthering Waves on MacOs."},
                    a { "It works by modifying the game's LocalStorage file (contains game settings) to set the maximum frame rate to 120 FPS." },
                    p { "Before starting, please verify that you have ran the game at least once to ensure that all necessary files are created." },
                    // p { "If you have any issues, please check the GitHub repository for more information." },
                    ExecuteUnlockFPSButton{}
                },
                div { id: "menu-content-main",
                }
                div { id: "footer",
                    img { src: GITHUB_SVG, id: "github-icon",
                    alt: "GitHub Icon",
                        width: "32",
                        height: "32"
                    },
                    div { id: "github-link",
                    a { "Source" }
                    }
                }
            }
        }
    }
}

#[component]
fn ExecuteUnlockFPSButton() -> Element {

    rsx! {
        button {
            class: "execute-unlock-button",
            onclick: move |_| {
                let update_result = crate::helpers::execute_unlock_fps();
            },
            "I understand, please proceed"
        }
    }
}
