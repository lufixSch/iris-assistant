#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::io::{self};
use std::sync::mpsc::{channel, Receiver, Sender};
use strum::IntoEnumIterator;

use iris::{self, Actions, IrisConfig};

enum AppStates {
    Init,
    Wait,
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
    let response_channel: (Sender<AppStates>, Receiver<AppStates>) = channel();

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

    let iris_config = match IrisConfig::load() { Ok(conf) => conf, Err(err) => {
        app_state  = AppStates::Error(err);
        IrisConfig::default()
    } };

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
                    let ctx = ui.ctx().clone();
                    let sender = response_channel.0.clone();
                    let context_ref = context.clone();
                    let action_ref = active_action.clone();
                    let user_input_ref = user_input.clone();
                    let iris_config_ref = iris_config.clone();

                    execute(move || {
                        let new_state = match iris::run(&action_ref, &context_ref, Some(&user_input_ref), iris_config_ref) {
                            Some(res) => AppStates::Response(res),
                            _ => AppStates::Error("Failed to generate response!".to_owned()),
                        };

                        let _ = sender.send(new_state);
                        ctx.request_repaint();
                    });

                    app_state = AppStates::Wait
                }
            }
            AppStates::Wait => {
                if let Ok(res) = response_channel.1.try_recv() {
                    app_state = res;
                }

                ui.label("Loading...");
            }
            AppStates::Response(res) => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    CommonMarkViewer::new().show(ui, &mut markdown_cache, res);
                });
            }
            AppStates::Error(err) => {
                ui.label(err);

                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    })
}

fn execute<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}
