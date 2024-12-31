use crate::brush::{BankArgs, CurveSelect, RaytoArgs};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Debug, Resource)]
pub struct OccupiedScreenSpace {
    right: f32,
}

#[derive(Default, Resource, Debug)]
pub struct GuiData {
    selected: Selected,
    rayto_args: RaytoArgs,
    bank_args: BankArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Selected {
    Rayto,
    Bank,
}

impl Default for Selected {
    fn default() -> Self {
        Self::Bank
    }
}

pub fn ui(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut curve_select: ResMut<CurveSelect>,
    mut local: Local<GuiData>,
) {
    let ctx = contexts.ctx_mut();
    occupied_screen_space.right = egui::SidePanel::right("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");

            egui::ComboBox::from_label("Curve")
                .selected_text(format!("{:?}", local.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut local.selected, Selected::Rayto, "Rayto");
                    ui.selectable_value(&mut local.selected, Selected::Bank, "Bank");
                });

            match local.selected {
                Selected::Rayto => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.n).speed(0.1));
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.r0).speed(0.1));
                        ui.label("r0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.r1).speed(0.1));
                        ui.label("r1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.theta0).speed(0.1));
                        ui.label("theta0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.theta1).speed(0.1));
                        ui.label("theta1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.x).speed(0.1));
                        ui.label("x");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.y).speed(0.1));
                        ui.label("y");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.h).speed(0.1));
                        ui.label("h");
                    });
                }
                Selected::Bank => (),
            }

            ui.label(format!("Selected curve is {:?}", *curve_select));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    *curve_select = match local.selected {
        Selected::Rayto => CurveSelect::Rayto(local.rayto_args.clone()),
        Selected::Bank => CurveSelect::Bank(local.bank_args.clone()),
    };
}
