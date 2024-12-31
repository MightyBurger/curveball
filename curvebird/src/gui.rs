use crate::brush::{BankArgs, CatenaryArgs, CurveSelect, EasySerpArgs, RaytoArgs, SerpentineArgs};

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
    catenary_args: CatenaryArgs,
    serpentine_args: SerpentineArgs,
    easyserp_args: EasySerpArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Selected {
    Rayto,
    Bank,
    Catenary,
    Serpentine,
    EasySerp,
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
                    ui.selectable_value(&mut local.selected, Selected::Rayto, "rayto");
                    ui.selectable_value(&mut local.selected, Selected::Bank, "bank");
                    ui.selectable_value(&mut local.selected, Selected::Catenary, "catenary");
                    ui.selectable_value(&mut local.selected, Selected::Serpentine, "serpentine");
                    ui.selectable_value(&mut local.selected, Selected::EasySerp, "easy-serp");
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
                Selected::Bank => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.n).speed(0.1));
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.ri0).speed(0.1));
                        ui.label("ri0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.ro0).speed(0.1));
                        ui.label("ro0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.ri1).speed(0.1));
                        ui.label("ri1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.ro1).speed(0.1));
                        ui.label("ro1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.theta0).speed(0.1));
                        ui.label("theta0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.theta1).speed(0.1));
                        ui.label("theta1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.h).speed(0.1));
                        ui.label("h");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.t).speed(0.1));
                        ui.label("t");
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut local.bank_args.fill, "fill");
                        //ui.label("fill");
                    });
                }
                Selected::Catenary => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.n).speed(0.1));
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.x0).speed(0.1));
                        ui.label("x0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.z0).speed(0.1));
                        ui.label("z0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.x1).speed(0.1));
                        ui.label("x1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.z1).speed(0.1));
                        ui.label("z1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.s).speed(0.1));
                        ui.label("s");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.w).speed(0.1));
                        ui.label("w");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.t).speed(0.1));
                        ui.label("t");
                    });
                    // TODO: allow user to change initial guess
                }
                Selected::Serpentine => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.n0).speed(0.1));
                        ui.label("n0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.n1).speed(0.1));
                        ui.label("n1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.x).speed(0.1));
                        ui.label("x");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.z).speed(0.1));
                        ui.label("z");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.xm).speed(0.1));
                        ui.label("xm");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.zm).speed(0.1));
                        ui.label("zm");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.w).speed(0.1));
                        ui.label("w");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.t).speed(0.1));
                        ui.label("t");
                    });
                }
                Selected::EasySerp => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.easyserp_args.n).speed(0.1));
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.easyserp_args.x).speed(0.1));
                        ui.label("x");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.easyserp_args.z).speed(0.1));
                        ui.label("z");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.easyserp_args.w).speed(0.1));
                        ui.label("w");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.easyserp_args.t).speed(0.1));
                        ui.label("t");
                    });
                }
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
        Selected::Catenary => CurveSelect::Catenary(local.catenary_args.clone()),
        Selected::Serpentine => CurveSelect::Serpentine(local.serpentine_args.clone()),
        Selected::EasySerp => CurveSelect::EasySerp(local.easyserp_args.clone()),
    };
}