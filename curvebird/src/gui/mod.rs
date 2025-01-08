// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::brush::{
    BankArgs, CatenaryArgs, CurveClassicArgs, CurveSelect, CurveSlopeArgs, MeshDisplaySettings,
    RaytoArgs, SerpentineArgs,
};
use crate::camera_controller::CameraController;
use crate::{GizmoSettings, MeshGen};

use bevy::prelude::*;
use bevy_egui::egui::containers::modal::Modal;
use bevy_egui::egui::{menu, Id};
use bevy_egui::{egui, EguiContexts};
use curveball::map::{QEntity, QMap, SimpleWorldspawn};

use egui_extras::{Column, TableBuilder};

mod curveopts;

#[derive(Default, Debug, Resource)]
pub struct OccupiedScreenSpace {
    top: f32,
    right: f32,
    bottom: f32,
}

#[derive(Default, Resource, Debug)]
pub struct GuiData {
    guide_open: bool,
    guide_example: f32,
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
    mut meshdisp: ResMut<MeshDisplaySettings>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mut gizmo_settings: ResMut<GizmoSettings>,
) {
    let Ok((mut cam_transform, mut cam_controller)) = query.get_single_mut() else {
        return;
    };

    let ctx = contexts.ctx_mut();

    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(false)
        .exact_width(200.0)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.label("Curve");
            egui::ComboBox::from_id_salt("CurveSelect")
                .selected_text(format!("{:?}", local.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut local.selected,
                        Selected::CurveClassic,
                        "Curve Classic",
                    );
                    ui.selectable_value(&mut local.selected, Selected::CurveSlope, "Curve Slope");
                    ui.selectable_value(&mut local.selected, Selected::Rayto, "Rayto");
                    ui.selectable_value(&mut local.selected, Selected::Bank, "Bank");
                    ui.selectable_value(&mut local.selected, Selected::Catenary, "Catenary");
                    ui.selectable_value(&mut local.selected, Selected::Serpentine, "Serpentine");
                });

            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                match local.selected {
                    Selected::CurveClassic => {
                        curveopts::curveclassic_ui(ui, &mut local.curveclassic_args)
                    }
                    Selected::CurveSlope => {
                        curveopts::curveslope_ui(ui, &mut local.curveslope_args)
                    }
                    Selected::Rayto => curveopts::rayto_ui(ui, &mut local.rayto_args),
                    Selected::Bank => curveopts::bank_ui(ui, &mut local.bank_args),
                    Selected::Catenary => curveopts::catenary_ui(ui, &mut local.catenary_args),
                    Selected::Serpentine => {
                        curveopts::serpentine_ui(ui, &mut local.serpentine_args)
                    }
                }

                ui.add_space(8.0);

                if ui
                    .button("Reset")
                    .on_hover_text("Reset the curve to default settings")
                    .clicked()
                {
                    match local.selected {
                        Selected::CurveClassic => {
                            local.curveclassic_args = CurveClassicArgs::default()
                        }
                        Selected::CurveSlope => local.curveslope_args = CurveSlopeArgs::default(),
                        Selected::Rayto => local.rayto_args = RaytoArgs::default(),
                        Selected::Bank => local.bank_args = BankArgs::default(),
                        Selected::Catenary => local.catenary_args = CatenaryArgs::default(),
                        Selected::Serpentine => local.serpentine_args = SerpentineArgs::default(),
                    }
                };
                ui.add_space(8.0);
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let clipboard_text = "Copy map to clipboard";
                    match &meshgen.0 {
                        Some(Ok(brushes)) => {
                            if ui.button(clipboard_text).on_hover_text("Copy the map to the clipboard. You can then paste the curve directly into your level in a program like Trenchbroom.").clicked() {
                                let simple_worldspawn = SimpleWorldspawn::new(brushes.clone());
                                let entity = QEntity::from(simple_worldspawn);
                                let map = QMap::new(vec![entity]).with_tb_neverball_metadata();
                                let mapstr = map.to_string();
                                write_to_clipboard(mapstr);
                                info!("Copied map to clipboard");
                                ui.close_menu();
                            };
                        }
                        _ => {
                            ui.add_enabled_ui(false, |ui| ui.button(clipboard_text));
                        }
                    };
                });
                ui.menu_button("View", |ui| {

                    ui
                        .checkbox(&mut gizmo_settings.show, "Show grid")
                        .on_hover_text("Show the grid and axis.");

                    ui.separator();

                    ui
                        .checkbox(&mut meshdisp.alternating_colors, "Alternating colors")
                        .on_hover_text("Shade every other brush in the curve a darker color.");

                    ui.separator();

                    let min_walkspeed = cam_controller.min_walkspeed;
                    let max_walkspeed = cam_controller.max_walkspeed;
                    ui.add(egui::Slider::new(&mut cam_controller.walk_speed, min_walkspeed..=max_walkspeed)
                        .text("Speed")
                        .logarithmic(true)
                        .show_value(false));
                    cam_controller.run_speed = cam_controller.walk_speed * cam_controller.run_factor;
                    if ui.button("Reset camera")
                        .on_hover_text("Reset the camera to its default location and speed.")
                        .clicked() {
                            *cam_transform =
                                Transform::from_xyz(256.0, 256.0, -384.0).looking_at(Vec3::ZERO, Vec3::Y);
                            *cam_controller = CameraController::default();
                            ui.close_menu();
                        }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("Guide").clicked() {
                        local.guide_open = true;
                    }
                });
            });
        })
        .response
        .rect
        .width();

    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let bottom_panel_left_to_right = egui::Layout {
                    main_dir: egui::Direction::LeftToRight,
                    main_wrap: false,
                    main_align: egui::Align::Max,
                    main_justify: false,
                    cross_align: egui::Align::Min,
                    cross_justify: false,
                };

                ui.with_layout(bottom_panel_left_to_right, |ui| {
                    match &meshgen.0 {
                        Some(Ok(brushes)) => {
                            let brushes_len = brushes.len();
                            let brushes_text = if brushes_len == 1 { "brush" } else { "brushes" };
                            let sides_len: usize = brushes
                                .iter()
                                .map(|brush| brush.to_sides_unique().len())
                                .sum();
                            let sides_text = if brushes_len == 1 { "side" } else { "sides" };
                            ui.label(format!(
                                "{brushes_len} {brushes_text} | {sides_len} {sides_text}"
                            ));
                        }
                        Some(Err(e)) => {
                            ui.label(format!("{}", e)).on_hover_text(
                                "An error is preventing the program from generating the curve.",
                            );
                        }
                        None => (),
                    };
                });
                let bottom_panel_right_to_left = egui::Layout {
                    main_dir: egui::Direction::RightToLeft,
                    main_wrap: false,
                    main_align: egui::Align::Max,
                    main_justify: false,
                    cross_align: egui::Align::Min,
                    cross_justify: false,
                };

                ui.with_layout(bottom_panel_right_to_left, |ui| {
                    ui.label(format!(
                        "{} {}",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION")
                    ));
                });
            });
        })
        .response
        .rect
        .width();

    if local.guide_open {
        let modal = Modal::new(Id::new("Guide Modal")).show(ctx, |ui| {
            ui.heading("Controls");
            ui.add_space(4.0);
            TableBuilder::new(ui)
                .id_salt("ControlsTable")
                .striped(true)
                .column(Column::auto().resizable(false))
                .column(Column::remainder())
                .body(|mut body| {
                    let row_len = 20.0;
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("W, A, S, D");
                        });
                        row.col(|ui| {
                            ui.label("Navigate");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Q, E");
                        });
                        row.col(|ui| {
                            ui.label("Move up and down");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Right Click + Drag");
                        });
                        row.col(|ui| {
                            ui.label("Look around");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Scroll up/down");
                        });
                        row.col(|ui| {
                            ui.label("Change camera speed when moving");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Shift");
                        });
                        row.col(|ui| {
                            ui.label("Hold to move faster");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("M");
                        });
                        row.col(|ui| {
                            ui.label("Toggle grabbing the mouse");
                        });
                    });
                });

            ui.add_space(4.0);
            ui.separator();

            ui.heading("Making the curve");
            ui.add_space(4.0);
            ui.label("Click on a number, then type in a new number to change it.");
            ui.label("You can also Left Click + Drag. Try it out!");
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut local.guide_example)
                        .speed(0.1)
                        .range(-100.0..=100.0),
                )
                .on_hover_text("Be careful, time traveler.");
                ui.label("Squimble");
            });

            ui.add_space(8.0);
            ui.separator();

            ui.heading("Status bar");
            ui.add_space(4.0);
            ui.label("The status bar is at the bottom of the screen.");
            ui.label("If your curve disappears, look there to see why.");

            ui.add_space(8.0);
            ui.separator();

            ui.heading("Putting the curve in your level");
            ui.add_space(4.0);
            ui.label("Go to File ➡ Copy map to clipboard.");
            ui.label("Then, inside Trenchbroom, press Ctrl + V to paste in your curve.");
        });

        if modal.should_close() {
            local.guide_open = false;
        }
    }
    // ui.horizontal(|ui| {
    //     ui.add(egui::DragValue::new(&mut args.ri1).speed(0.1))
    //         .on_hover_text("ri1");
    //     ui.label("Inner radius");
    // });

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
