use crate::brush::{BankArgs, CurveSelect, RaytoArgs};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Debug, Resource)]
pub struct OccupiedScreenSpace {
    right: f32,
}

pub fn ui(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut curve_select: ResMut<CurveSelect>,
) {
    let ctx = contexts.ctx_mut();
    occupied_screen_space.right = egui::SidePanel::right("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            if ui.button("Here is a button").clicked() {
                info!("clicked");
                let next_curve = match *curve_select {
                    CurveSelect::Rayto(_) => CurveSelect::Bank(BankArgs::default()),
                    CurveSelect::Bank(_) => CurveSelect::Rayto(RaytoArgs::default()),
                };
                *curve_select = next_curve;
            };
            ui.label(format!("Selected curve is {:?}", *curve_select));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
