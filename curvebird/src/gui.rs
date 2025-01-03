// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::brush::{
    BankArgs, CatenaryArgs, CurveClassicArgs, CurveSelect, CurveSlopeArgs, RaytoArgs,
    SerpentineArgs,
};
use crate::MeshGen;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use curveball::map::{QEntity, QMap, SimpleWorldspawn};

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
        .resizable(true)
        .show(ctx, |ui| {
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
                Selected::CurveClassic => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.n).speed(0.1))
                            .on_hover_text("Number of segments");
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.ri0).speed(0.1))
                            .on_hover_text("Starting inner radius");
                        ui.label("ri0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.ro0).speed(0.1))
                            .on_hover_text("Starting outer raidus");
                        ui.label("ro0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.ri1).speed(0.1))
                            .on_hover_text("Ending inner radius");
                        ui.label("ri1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.ro1).speed(0.1))
                            .on_hover_text("Ending outer radius");
                        ui.label("ro1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.theta0).speed(0.1))
                            .on_hover_text("Starting angle (deg)");
                        ui.label("theta0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.theta1).speed(0.1))
                            .on_hover_text("Ending angle (deg)");
                        ui.label("theta1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveclassic_args.t).speed(0.1))
                            .on_hover_text("Thickness");
                        ui.label("t");
                    });
                }

                Selected::CurveSlope => {
                    ui.add_space(8.0);
                    ui.label("General");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.n).speed(0.1))
                            .on_hover_text("Number of segments");
                        ui.label("n");
                    });
                    ui.add_space(8.0);
                    ui.label("Radii");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.ri0).speed(0.1))
                            .on_hover_text("Starting inner radius");
                        ui.label("ri0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.ro0).speed(0.1))
                            .on_hover_text("Starting outer raidus");
                        ui.label("ro0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.ri1).speed(0.1))
                            .on_hover_text("Ending inner radius");
                        ui.label("ri1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.ro1).speed(0.1))
                            .on_hover_text("Ending outer radius");
                        ui.label("ro1");
                    });
                    ui.add_space(8.0);
                    ui.label("Angles");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.theta0).speed(0.1))
                            .on_hover_text("Starting angle (deg)");
                        ui.label("theta0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.theta1).speed(0.1))
                            .on_hover_text("Ending angle (deg)");
                        ui.label("theta1");
                    });
                    ui.add_space(8.0);
                    ui.label("Heights");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.t).speed(0.1))
                            .on_hover_text("Thickness");
                        ui.label("t");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.slope).speed(0.1))
                            .on_hover_text("Slope");
                        ui.label("slope");
                    });
                    ui.add_space(8.0);
                    ui.label("Drops");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_inner_top_0).speed(0.1))
                            .on_hover_text("Starting inner drop, top");
                        ui.label("drop_inner_top_0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_inner_bot_0).speed(0.1))
                            .on_hover_text("Starting inner drop, bottom");
                        ui.label("drop_inner_bot_0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_outer_top_0).speed(0.1))
                            .on_hover_text("Starting outer drop, top");
                        ui.label("drop_outer_top_0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_outer_bot_0).speed(0.1))
                            .on_hover_text("Starting outer drop, bottom");
                        ui.label("drop_outer_bot_0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_inner_top_1).speed(0.1))
                            .on_hover_text("Ending inner drop, top");
                        ui.label("drop_inner_top_1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_inner_bot_1).speed(0.1))
                            .on_hover_text("Ending inner drop, bottom");
                        ui.label("drop_inner_bot_1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_outer_top_1).speed(0.1))
                            .on_hover_text("Ending outer drop, top");
                        ui.label("drop_outer_top_1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.drop_outer_bot_1).speed(0.1))
                            .on_hover_text("Ending outer drop, bottom");
                        ui.label("drop_outer_bot_1");
                    });
                    ui.add_space(8.0);
                    ui.label("Hills");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.hill_inner_top).speed(0.1))
                            .on_hover_text("Inner hill, top");
                        ui.label("hill_inner_top");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.hill_inner_bot).speed(0.1))
                            .on_hover_text("Inner hill, bottom");
                        ui.label("hill_inner_bot");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.hill_outer_top).speed(0.1))
                            .on_hover_text("Outer hill, top");
                        ui.label("hill_outer_top");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.curveslope_args.hill_outer_bot).speed(0.1))
                            .on_hover_text("Outer hill, bottom");
                        ui.label("hill_outer_bot");
                    });
                }

                Selected::Rayto => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.n).speed(0.1))
                            .on_hover_text("Number of segments");
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.r0).speed(0.1))
                            .on_hover_text("Starting radius");
                        ui.label("r0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.r1).speed(0.1))
                            .on_hover_text("Ending radius");
                        ui.label("r1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.theta0).speed(0.1))
                            .on_hover_text("Starting angle (deg)");
                        ui.label("theta0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.theta1).speed(0.1))
                            .on_hover_text("Ending angle (deg)");
                        ui.label("theta1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.x).speed(0.1))
                            .on_hover_text("X-coordinate of corner");
                        ui.label("x");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.y).speed(0.1))
                            .on_hover_text("Y-coordinate of corner");
                        ui.label("y");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.rayto_args.h).speed(0.1))
                            .on_hover_text("Height/thickness of curve");
                        ui.label("h");
                    });
                }

                Selected::Bank => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.n).speed(0.1))
                            .on_hover_text("Number of segments");
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.ri).speed(0.1))
                            .on_hover_text("Inner radius");
                        ui.label("ri");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.ro).speed(0.1))
                            .on_hover_text("Outer radius");
                        ui.label("ro");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.theta0).speed(0.1))
                            .on_hover_text("Starting angle (deg)");
                        ui.label("theta0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.theta1).speed(0.1))
                            .on_hover_text("Ending angle (deg)");
                        ui.label("theta1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.h).speed(0.1))
                            .on_hover_text("Cone height");
                        ui.label("h");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.bank_args.t).speed(0.1))
                            .on_hover_text("Thickness of the bank");
                        ui.label("t");
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut local.bank_args.fill, "fill")
                            .on_hover_text("Filled bank");
                    });
                }

                Selected::Catenary => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.n).speed(0.1))
                            .on_hover_text("Number of segments");
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.x0).speed(0.1))
                            .on_hover_text("Starting horizontal position of the curve");
                        ui.label("x0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.z0).speed(0.1))
                            .on_hover_text("Starting height of the curve");
                        ui.label("z0");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.x1).speed(0.1))
                            .on_hover_text("Ending horizontal position of the curve");
                        ui.label("x1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.z1).speed(0.1))
                            .on_hover_text("Ending height of the curve");
                        ui.label("z1");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.s).speed(0.1))
                            .on_hover_text("Length of the curve (i.e. how long your rope is)");
                        ui.label("s");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.w).speed(0.1))
                            .on_hover_text("Width of the curve");
                        ui.label("w");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.catenary_args.t).speed(0.1))
                            .on_hover_text("Thickness of the curve");
                        ui.label("t");
                    });
                    // TODO: allow user to change initial guess
                }

                Selected::Serpentine => {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.n).speed(0.1))
                            .on_hover_text("Number of segments; will be rounded up to the nearest multiple of 2");
                        ui.label("n");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.x).speed(0.1))
                            .on_hover_text("Ending horizontal position of the curve");
                        ui.label("x");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.z).speed(0.1))
                            .on_hover_text("Ending height of the curve");
                        ui.label("z");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.w).speed(0.1))
                            .on_hover_text("Width of the curve");
                        ui.label("w");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut local.serpentine_args.t).speed(0.1))
                            .on_hover_text("Thickness of the curve");
                        ui.label("t");
                    });

                }
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
