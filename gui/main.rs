#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Align, Button, Layout, Pos2, Rect};
use eframe::emath::align;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::io::{self};
use std::sync::mpsc::{channel, Receiver, Sender};
use strum::IntoEnumIterator;

use iris::{self, Actions, IrisConfig};

pub mod shortcuts;
use shortcuts::check_action_shortcuts;

/// Represents the different states of the application.
enum AppStates {
    Init,
    Wait,
    Error(String),
    Response(String),
}

/// The main function that initializes and runs the eframe application.
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut markdown_cache = CommonMarkCache::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([260.0, 260.0]),
        ..Default::default()
    };

    let (sender, receiver) = channel();
    let mut app_state = AppStates::Init;
    let mut active_action = Actions::Explain;
    let mut user_input = String::new();
    let mut context = String::new();

    if let Err(err) = read_context_from_stdin(&mut context) {
        app_state = AppStates::Error(err);
    }

    let iris_config = IrisConfig::load().unwrap_or_else(|err| {
        app_state = AppStates::Error(err);
        IrisConfig::default()
    });

    eframe::run_simple_native("Iris", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().url_in_tooltip = true;

            match &app_state {
                AppStates::Init => render_init_ui(
                    ui,
                    &mut active_action,
                    &mut app_state,
                    &mut user_input,
                    sender.clone(),
                    context.clone(),
                    iris_config.clone(),
                ),
                AppStates::Wait => render_wait_ui(ui, &receiver, &mut app_state),
                AppStates::Response(res) => render_response_ui(ui, res, &mut markdown_cache),
                AppStates::Error(err) => render_error_ui(ui, err),
            }
        });
    })
}

/// Reads context from standard input and stores it in the provided string.
fn read_context_from_stdin(context: &mut String) -> Result<(), String> {
    let stdin = io::stdin();
    for line in stdin.lines() {
        *context += "\n";
        context.push_str(&line.map_err(|_| "Could not read line from standard input.".to_owned())?);
    }
    Ok(())
}

/// Renders the initial UI where the user can select an action and provide input.
fn render_init_ui(
    ui: &mut egui::Ui,
    active_action: &mut Actions,
    app_state: &mut AppStates,
    user_input: &mut String,
    sender: Sender<AppStates>,
    context: String,
    iris_config: IrisConfig,
) {
    // Check Shortcuts
    check_action_shortcuts(ui.ctx(), active_action);

    ui.horizontal(|ui| {
        for action in Actions::iter() {
            ui.radio_value(active_action, action.clone(), action.to_string());
        }
    });

    if *active_action == Actions::Edit || *active_action == Actions::Ask {
        let hint = match active_action {
            Actions::Edit => "Rewrite in a professional tone.",
            Actions::Ask => "What is the distance between the earth and the moon?",
            _ => "",
        };
        ui.add_sized(
            ui.available_size() - egui::Vec2 { x: 0.0, y: 25.0 },
            egui::TextEdit::multiline(user_input).hint_text(hint),
        );
    }

    ui.vertical_centered(|ui| {
        if ui.button("Send").clicked() {
            let ctx = ui.ctx().clone();
            let context_ref = context.clone();
            let action_ref = active_action.clone();
            let user_input_ref = user_input.clone();
            let iris_config_ref = iris_config.clone();

            execute(move || {
                let new_state = match iris::run(
                    &action_ref,
                    &context_ref,
                    Some(&user_input_ref),
                    iris_config_ref,
                ) {
                    Some(res) => AppStates::Response(res),
                    _ => AppStates::Error("Failed to generate response!".to_owned()),
                };

                let _ = sender.send(new_state);
                ctx.request_repaint();
            });

            *app_state = AppStates::Wait;
        }
    });
}

/// Renders the UI while waiting for a response from the iris service.
fn render_wait_ui(ui: &mut egui::Ui, receiver: &Receiver<AppStates>, app_state: &mut AppStates) {
    if let Ok(res) = receiver.try_recv() {
        *app_state = res;
    }

    ui.vertical_centered(|ui| {
        ui.label("Loading...");
    });
}

/// Renders the response received from the iris service.
fn render_response_ui(ui: &mut egui::Ui, response: &str, markdown_cache: &mut CommonMarkCache) {
    // ui.with_layout(Layout::left_to_right( Align::Max), |ui| {
    if ui.button("📋").clicked() {
        ui.ctx().copy_text(response.to_owned());
    }
    // });

    egui::ScrollArea::vertical().show(ui, |ui| {
        CommonMarkViewer::new().show(ui, markdown_cache, response);
    });
}

/// Renders an error message and provides an exit button.
fn render_error_ui(ui: &mut egui::Ui, error: &str) {
    ui.label(error);

    if ui.button("Exit").clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

/// Executes a function in a separate thread.
fn execute<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}
