use dioxus::prelude::*;

const SVG: &str = include_str!("../../assets/github.svg");

#[derive(Clone, PartialEq)]
pub enum DialogType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct DialogState {
    pub show: bool,
    pub message: String,
    pub dialog_type: DialogType,
}

impl Default for DialogState {
    fn default() -> Self {
        Self {
            show: false,
            message: String::new(),
            dialog_type: DialogType::Success,
        }
    }
}

#[component]
pub fn Menu() -> Element {
    let dialog_state = use_signal(|| DialogState::default());

    rsx! {
        div { id: "menu",
            div { id: "menu-content",
                div { id: "menu-content-title",
                    p { "This application is a simple FPS unlocker specifically for Wuthering Waves on MacOs."},
                    a { "It works by modifying the game's LocalStorage file (contains game settings) to set the maximum frame rate to 120 FPS." },
                    p { "Before starting, please verify that you have ran the game at least once to ensure that all necessary files are created." },
                    ExecuteUnlockFPSButton {
                        dialog_state: dialog_state
                    }
                },
                div { id: "menu-content-main" }
                div { id: "footer",
                    // Use dangerous_inner_html to insert SVG as SVG rendered as ? when in desktop
                    // As opposed to web
                    // img { 
                    //     src: SVG, 
                    //     id: "github-icon",
                    //     alt: "GitHub Icon",
                    //     width: "32",
                    //     height: "32"
                    // },
                    div { 
                        class: "github-icon",
                        dangerous_inner_html: SVG
                    },
                    div { id: "github-link",
                        a { href: "https://github.com/Peekaey/MacOs-WuWa-fps-unlocker", class: "github-link" ,"Source code (v1.0.0) ❤️ " }
                    }
                }
            }
        }
        
        if dialog_state.read().show {
            DialogModal {
                dialog_state: dialog_state,
                message: dialog_state.read().message.clone(),
                dialog_type: dialog_state.read().dialog_type.clone(),
            }
        }
    }
}

#[component]
fn ExecuteUnlockFPSButton(dialog_state: Signal<DialogState>) -> Element {
    rsx! {
        button {
            class: "execute-unlock-button",
            onclick: move |_| {
                // Execute the unlock FPS function
                let update_result = crate::helpers::execute_unlock_fps();
                
                // Display result
                match update_result {
                    Ok(_) => {
                        dialog_state.write().show = true;
                        dialog_state.write().message = "FPS Cap has been set to 120 FPS".to_string();
                        dialog_state.write().dialog_type = DialogType::Success;
                    },
                    Err(e) => {
                        dialog_state.write().show = true;
                        dialog_state.write().message = format!("{}", e);
                        dialog_state.write().dialog_type = DialogType::Error;
                    }
                }
            },
            "I understand. please proceed"
        }
    }
}

#[component]
fn DialogModal(
    dialog_state: Signal<DialogState>,
    message: String,
    dialog_type: DialogType,
) -> Element {
    let dialog_class = match dialog_type {
        DialogType::Success => "dialog-success",
        DialogType::Error => "dialog-error",
    };

    let title = match dialog_type {
        DialogType::Success => "Success",
        DialogType::Error => "Error",
    };

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| {
                dialog_state.write().show = false;
            },
            
            div {
                class: "modal-dialog {dialog_class}",
                onclick: |e| e.stop_propagation(),
                
                div { class: "modal-header",
                    h3 { "{title}" }
                    button {
                        class: "modal-close",
                        onclick: move |_| {
                            dialog_state.write().show = false;
                        },
                        "×"
                    }
                }
                
                div { class: "modal-body",
                    p { "{message}" }
                }
                
                div { class: "modal-footer",
                    button {
                        class: "modal-button",
                        onclick: move |_| {
                            dialog_state.write().show = false;
                        },
                        "OK"
                    }
                }
            }
        }
    }
}
