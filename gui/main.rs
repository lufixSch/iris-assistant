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
        egui::CentralPanel::default().show(ctx, |ui| match &app_state {
            AppStates::Init => render_init_ui(ui, &mut active_action, &mut app_state, &mut user_input, sender.clone(), context.clone(), iris_config.clone()),
            AppStates::Wait => render_wait_ui(ui, &receiver, &mut app_state),
            AppStates::Response(res) => render_response_ui(ui, res, &mut markdown_cache),
            AppStates::Error(err) => render_error_ui(ui, err),
        });
    })
}

fn read_context_from_stdin(context: &mut String) -> Result<(), String> {
    let stdin = io::stdin();
    for line in stdin.lines() {
        *context += "\n";
        context.push_str(&line.map_err(|_| "Could not read line from standard input.".to_owned())?);
    }
    Ok(())
}

fn render_init_ui(ui: &mut egui::Ui, active_action: &mut Actions, app_state: &mut AppStates, user_input: &mut String, sender: Sender<AppStates>, context: String, iris_config: IrisConfig) {
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
            [ui.available_size()[0], 200.0],
            egui::TextEdit::multiline(user_input).hint_text(hint),
        );
    }

    if ui.button("Send").clicked() {
        let ctx = ui.ctx().clone();
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

        *app_state = AppStates::Wait;
    }
}

fn render_wait_ui(ui: &mut egui::Ui, receiver: &Receiver<AppStates>, app_state: &mut AppStates) {
    if let Ok(res) = receiver.try_recv() {
        *app_state = res;
    }

    ui.label("Loading...");
}

fn render_response_ui(ui: &mut egui::Ui, response: &str, markdown_cache: &mut CommonMarkCache) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        CommonMarkViewer::new().show(ui, markdown_cache, response);
    });
}

fn render_error_ui(ui: &mut egui::Ui, error: &str) {
    ui.label(error);

    if ui.button("Exit").clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

fn execute<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}
