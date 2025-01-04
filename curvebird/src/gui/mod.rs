// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::brush::{
    BankArgs, CatenaryArgs, CurveClassicArgs, CurveSelect, CurveSlopeArgs, RaytoArgs,
    SerpentineArgs,
};
use crate::MeshGen;

use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContexts};
use curveball::map::{QEntity, QMap, SimpleWorldspawn};

mod curveopts;

#[derive(Default, Debug, Resource)]
pub struct OccupiedScreenSpace {
    right: f32,
}

#[derive(Default, Resource, Debug)]
pub struct GuiData {
    selected: Selected,
    curveclassic_args: CurveClassicArgs,
    curveslope_args: CurveSlopeArgs,
    rayto_args: RaytoArgs,
    bank_args: BankArgs,
    catenary_args: CatenaryArgs,
    serpentine_args: SerpentineArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Selected {
    CurveClassic,
    CurveSlope,
    Rayto,
    Bank,
    Catenary,
    Serpentine,
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
    meshgen: Res<MeshGen>,
) {
    let ctx = contexts.ctx_mut();
    occupied_screen_space.right = egui::SidePanel::right("left_panel")
        .resizable(false)
        .exact_width(200.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(8.0);
                egui::ComboBox::from_id_salt("CurveSelect")
                    .selected_text(format!("{:?}", local.selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut local.selected, Selected::CurveClassic, "Curve Classic");
                        ui.selectable_value(&mut local.selected, Selected::CurveSlope, "Curve Slope");
                        ui.selectable_value(&mut local.selected, Selected::Rayto, "Rayto");
                        ui.selectable_value(&mut local.selected, Selected::Bank, "Bank");
                        ui.selectable_value(&mut local.selected, Selected::Catenary, "Catenary");
                        ui.selectable_value(&mut local.selected, Selected::Serpentine, "Serpentine");
                    });

                ui.separator();

                match local.selected {
                    Selected::CurveClassic => curveopts::curveclassic_ui(ui, &mut local.curveclassic_args),
                    Selected::CurveSlope => curveopts::curveslope_ui(ui, &mut local.curveslope_args),
                    Selected::Rayto => curveopts::rayto_ui(ui, &mut local.rayto_args),
                    Selected::Bank => curveopts::bank_ui(ui, &mut local.bank_args),
                    Selected::Catenary => curveopts::catenary_ui(ui, &mut local.catenary_args),
                    Selected::Serpentine => curveopts::serpentine_ui(ui, &mut local.serpentine_args),
                }

                ui.add_space(8.0);

                if ui.button("Reset").on_hover_text("Reset the curve to default settings").clicked() {
                    match local.selected {
                        Selected::CurveClassic => local.curveclassic_args = CurveClassicArgs::default(),
                        Selected::CurveSlope => local.curveslope_args = CurveSlopeArgs::default(),
                        Selected::Rayto => local.rayto_args = RaytoArgs::default(),
                        Selected::Bank => local.bank_args = BankArgs::default(),
                        Selected::Catenary => local.catenary_args = CatenaryArgs::default(),
                        Selected::Serpentine => local.serpentine_args = SerpentineArgs::default(),
                    }
                };
                ui.separator();

                let bottom_panel_layout = egui::Layout {
                    main_dir: egui::Direction::BottomUp,
                    main_wrap: true,
                    main_align: egui::Align::Max,
                    main_justify: false,
                    cross_align: egui::Align::Min,
                    cross_justify: false,
                };

                ui.with_layout(bottom_panel_layout, |ui| {
                    ui.add_space(8.0);
                    ui.label(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
                    ui.separator();
                    match &meshgen.0 {
                        Some(Ok(brushes)) => {
                            if ui.button("Copy map to clipboard").on_hover_text("Copy the map to the clipboard. You can then paste the curve directly into your level in a program like Trenchbroom.").clicked() {
                                let simple_worldspawn = SimpleWorldspawn::new(brushes.clone());
                                let entity = QEntity::from(simple_worldspawn);
                                let map = QMap::new(vec![entity]).with_tb_neverball_metadata();
                                let mapstr = map.to_string();
                                write_to_clipboard(mapstr);
                                info!("Copied map to clipboard");
                            };
                            // TODO: Implement Save to File
                            //if ui.button("Save to File").clicked() {};
                        }
                        Some(Err(e)) => {
                            ui.label(format!("{}", e)).on_hover_text("An error is preventing the program from generating the curve.");
                        }
                        None => (),
                    };
                });

            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    *curve_select = match local.selected {
        Selected::CurveClassic => CurveSelect::CurveClassic(local.curveclassic_args.clone()),
        Selected::CurveSlope => CurveSelect::CurveSlope(local.curveslope_args.clone()),
        Selected::Rayto => CurveSelect::Rayto(local.rayto_args.clone()),
        Selected::Bank => CurveSelect::Bank(local.bank_args.clone()),
        Selected::Catenary => CurveSelect::Catenary(local.catenary_args.clone()),
        Selected::Serpentine => CurveSelect::Serpentine(local.serpentine_args.clone()),
    };
}

fn write_to_clipboard(string: String) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use copypasta::{ClipboardContext, ClipboardProvider};
        let mut clip_ctx = ClipboardContext::new().unwrap();
        clip_ctx.set_contents(string).unwrap();
        // Bizarrely, this is requird to copy to clipboard on Ubuntu.
        // Probably a bug with copypasta.
        let _ = clip_ctx.get_contents().unwrap();
    }

    #[cfg(target_arch = "wasm32")]
    {
        let pool = bevy::tasks::TaskPool::new();
        pool.spawn_local(async move {
            let window = web_sys::window().expect("window");
            let nav = window.navigator().clipboard();
            let p = nav.write_text(&string);
            if let Err(_e) = wasm_bindgen_futures::JsFuture::from(p).await {
                warn!("error pasting to clipboard");
            };
        });
    }
}
