#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use log::{debug, error, info};
use std::io::{self, Read};
use strum::IntoEnumIterator;

use iris::{self, Actions};

enum AppStates {
    Init,
    Error(String),
    Response(String),
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut markdown_cache = CommonMarkCache::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([260.0, 260.0]),
        ..Default::default()
    };

    let mut app_state = AppStates::Init;

    // Our application state:
    let mut active_action = Actions::Explain;
    let mut user_input = "".to_owned();

    let stdin = io::stdin();
    let mut context = String::new();

    for line in stdin.lines() {
        let line = line.expect("Could not read line from standard in.");

        context += "\n";
        context.push_str(&line)
    }

    eframe::run_simple_native("Iris", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| match &app_state {
            AppStates::Init => {
                ui.horizontal(|ui| {
                    for action in Actions::iter() {
                        ui.radio_value(&mut active_action, action, action.to_string());
                    }
                });

                if active_action == Actions::Edit || active_action == Actions::Ask {
                    let hint = if active_action == Actions::Edit {
                        "Rewrite in a professional tone."
                    } else {
                        "What is the distance between the earth and the moon?"
                    };
                    ui.add_sized(
                        [ui.available_size()[0], 200.0],
                        egui::TextEdit::multiline(&mut user_input).hint_text(hint),
                    );
                }

                if ui.button("Send").clicked() {
                    app_state = match iris::run(&active_action, &context, Some(&user_input)) {
                        Some(res) => AppStates::Response(res),
                        _ => AppStates::Error("Failed to generate response!".to_owned()),
                    }
                }
            }
            AppStates::Response(res) => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    CommonMarkViewer::new().show(ui, &mut markdown_cache, res);
                });
            }
            AppStates::Error(err) => {
                ui.label(err);

                if ui.button("Try Again").clicked() {
                    app_state = AppStates::Init
                }
            }
        });
    })
}
