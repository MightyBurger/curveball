// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::brush::MeshDisplaySettings;
use crate::camera_controller::CameraController;
use crate::curveargs::{
    BankArgs, CatenaryArgs, CurveClassicArgs, CurveSelect, CurveSlopeArgs, ExtrusionArgs,
    PathLineArgs, PathRevolveArgs, PathSelect, ProfileAnnulusArgs, ProfileCircleArgs,
    ProfileRectangleArgs, ProfileSelect, RaytoArgs, SerpentineArgs,
};
use crate::{GizmoSettings, MeshGen};
use bevy::prelude::*;
use bevy_egui::egui::containers::modal::Modal;
use bevy_egui::egui::{Id, menu};
use bevy_egui::{EguiContexts, egui};
use curveball_lib::curve::extrude::ProfileOrientation;
use curveball_lib::map::entity::SimpleWorldspawn;
use curveball_lib::map::qmap::{QEntity, QMap};

use egui_extras::{Column, TableBuilder};

mod curveopts;
pub mod egui_blocking_plugin;

#[derive(Default, Debug, Resource)]
pub struct UiScreenState {
    pub top_panel_height: f32,
    pub right_panel_width: f32,
    pub bottom_panel_height: f32,
}

#[derive(Default, Resource, Debug)]
pub struct GuiData {
    controls_open: bool,
    guide_open: bool,
    guide_example: f32,
    about_open: bool,
    selected_curve: SelectedCurve,
    curveclassic_args: CurveClassicArgs,
    curveslope_args: CurveSlopeArgs,
    rayto_args: RaytoArgs,
    bank_args: BankArgs,
    catenary_args: CatenaryArgs,
    serpentine_args: SerpentineArgs,
    extrusion_gui_data: ExtrusionGuiData,
}

#[derive(Default, Resource, Debug)]
pub struct ExtrusionGuiData {
    selected_profile: SelectedProfile,
    profile_circle_args: ProfileCircleArgs,
    profile_rectangle_args: ProfileRectangleArgs,
    profile_annulus_args: ProfileAnnulusArgs,
    selected_path: SelectedPath,
    path_line_args: PathLineArgs,
    path_revolve_args: PathRevolveArgs,
    profile_orientation: ProfileOrientation,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelectedCurve {
    CurveClassic,
    CurveSlope,
    Rayto,
    Bank,
    Catenary,
    Serpentine,
    Extrusion,
}

impl Default for SelectedCurve {
    fn default() -> Self {
        Self::Bank
    }
}

impl std::fmt::Display for SelectedCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurveClassic => write!(f, "Curve Classic"),
            Self::CurveSlope => write!(f, "Curve Slope"),
            Self::Rayto => write!(f, "Rayto"),
            Self::Bank => write!(f, "Bank"),
            Self::Catenary => write!(f, "Catenary"),
            Self::Serpentine => write!(f, "Serpentine"),
            Self::Extrusion => write!(f, "Extrusion"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelectedProfile {
    Circle,
    Rectangle,
    Annulus,
}

impl std::fmt::Display for SelectedProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::Rectangle => write!(f, "Rectangle"),
            Self::Annulus => write!(f, "Annulus"),
        }
    }
}

impl Default for SelectedProfile {
    fn default() -> Self {
        Self::Circle
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelectedPath {
    Line,
    Revolve,
}

impl std::fmt::Display for SelectedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line => write!(f, "Line"),
            Self::Revolve => write!(f, "Revolve"),
        }
    }
}

impl Default for SelectedPath {
    fn default() -> Self {
        Self::Revolve
    }
}

pub fn ui(
    mut contexts: EguiContexts,
    mut ui_screen_state: ResMut<UiScreenState>,
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

    ui_screen_state.right_panel_width = egui::SidePanel::right("right_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.label("Curve");
            egui::ComboBox::from_id_salt("CurveSelect")
                .selected_text(format!("{}", local.selected_curve))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut local.selected_curve,
                        SelectedCurve::CurveClassic,
                        "Curve Classic",
                    );
                    ui.selectable_value(
                        &mut local.selected_curve,
                        SelectedCurve::CurveSlope,
                        "Curve Slope",
                    );
                    ui.selectable_value(&mut local.selected_curve, SelectedCurve::Rayto, "Rayto");
                    ui.selectable_value(&mut local.selected_curve, SelectedCurve::Bank, "Bank");
                    ui.selectable_value(
                        &mut local.selected_curve,
                        SelectedCurve::Catenary,
                        "Catenary",
                    );
                    ui.selectable_value(
                        &mut local.selected_curve,
                        SelectedCurve::Serpentine,
                        "Serpentine",
                    );
                    ui.selectable_value(
                        &mut local.selected_curve,
                        SelectedCurve::Extrusion,
                        "Extrusion",
                    );
                });

            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                match local.selected_curve {
                    SelectedCurve::CurveClassic => {
                        curveopts::curveclassic_ui(ui, &mut local.curveclassic_args)
                    }
                    SelectedCurve::CurveSlope => {
                        curveopts::curveslope_ui(ui, &mut local.curveslope_args)
                    }
                    SelectedCurve::Rayto => curveopts::rayto_ui(ui, &mut local.rayto_args),
                    SelectedCurve::Bank => curveopts::bank_ui(ui, &mut local.bank_args),
                    SelectedCurve::Catenary => curveopts::catenary_ui(ui, &mut local.catenary_args),
                    SelectedCurve::Serpentine => {
                        curveopts::serpentine_ui(ui, &mut local.serpentine_args)
                    }
                    SelectedCurve::Extrusion => {
                        curveopts::extrusion_ui(ui, &mut local.extrusion_gui_data)
                    }
                }

                ui.separator();

                if ui
                    .button("Reset")
                    .on_hover_text("Reset the curve to default settings")
                    .clicked()
                {
                    match local.selected_curve {
                        SelectedCurve::CurveClassic => {
                            local.curveclassic_args = CurveClassicArgs::default()
                        }
                        SelectedCurve::CurveSlope => {
                            local.curveslope_args = CurveSlopeArgs::default()
                        }
                        SelectedCurve::Rayto => local.rayto_args = RaytoArgs::default(),
                        SelectedCurve::Bank => local.bank_args = BankArgs::default(),
                        SelectedCurve::Catenary => local.catenary_args = CatenaryArgs::default(),
                        SelectedCurve::Serpentine => {
                            local.serpentine_args = SerpentineArgs::default()
                        }
                        SelectedCurve::Extrusion => {
                            local.extrusion_gui_data = ExtrusionGuiData::default()
                        }
                    }
                };
                ui.add_space(8.0);
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    ui_screen_state.top_panel_height = egui::TopBottomPanel::top("top_panel")
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

                    let min_walkspeed = cam_controller.settings.min_walkspeed;
                    let max_walkspeed = cam_controller.settings.max_walkspeed;
                    ui.add(egui::Slider::new(&mut cam_controller.walk_speed, min_walkspeed..=max_walkspeed)
                        .text("Speed")
                        .logarithmic(true)
                        .show_value(false));
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
                    if ui.button("Controls").clicked() {
                        local.controls_open = true;
                    }
                    if ui.button("Guide").clicked() {
                        local.guide_open = true;
                    }
                    if ui.button("About Curveball").clicked() {
                        local.about_open = true;
                    }
                });
            });
        })
        .response
        .rect
        .height();

    ui_screen_state.bottom_panel_height = egui::TopBottomPanel::bottom("bottom_panel")
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
                            let sides_text = if sides_len == 1 { "side" } else { "sides" };
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
        .height();

    if local.controls_open {
        let modal = Modal::new(Id::new("Controls Modal")).show(ctx, |ui| {
            ui.heading("Controls");
            ui.add_space(4.0);
            TableBuilder::new(ui)
                .id_salt("MouseControlsTable")
                .striped(true)
                .column(Column::exact(120.0).resizable(false))
                .column(Column::remainder())
                .body(|mut body| {
                    let row_len = 20.0;
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Left click");
                        });
                        row.col(|ui| {
                            ui.label("Orbit around origin");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Middle click");
                        });
                        row.col(|ui| {
                            ui.label("Pan");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Right click");
                        });
                        row.col(|ui| {
                            ui.label("Look around");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Alt + Right click");
                        });
                        row.col(|ui| {
                            ui.label("Orbit around point");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Scroll");
                        });
                        row.col(|ui| {
                            ui.label("Zoom");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("Scroll while moving");
                        });
                        row.col(|ui| {
                            ui.label("Change movement speed");
                        });
                    });
                });
            ui.separator();
            ui.add_space(4.0);
            TableBuilder::new(ui)
                .id_salt("KeyboardControlsTable")
                .striped(true)
                .column(Column::exact(120.0).resizable(false))
                .column(Column::remainder())
                .body(|mut body| {
                    let row_len = 20.0;
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("W, A, S, D");
                        });
                        row.col(|ui| {
                            ui.label("Move around");
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
                            ui.label("Shift");
                        });
                        row.col(|ui| {
                            ui.label("Hold to move faster");
                        });
                    });
                    body.row(row_len, |mut row| {
                        row.col(|ui| {
                            ui.label("C");
                        });
                        row.col(|ui| {
                            ui.label("Toggle grabbing the mouse");
                        });
                    });
                });
        });

        if modal.should_close() {
            local.controls_open = false;
        }
    }

    if local.guide_open {
        let modal = Modal::new(Id::new("Guide Modal")).show(ctx, |ui| {
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

    if local.about_open {
        let modal = Modal::new(Id::new("About Modal")).show(ctx, |ui| {
            let version = env!("CARGO_PKG_VERSION");
            ui.heading(format!("Curveball"));
            ui.label(format!("{version}"));
            ui.add_space(12.0);
            ui.label("Curveball is a curve generator for Neverball levels.");
            ui.add_space(12.0);
            ui.label("Curveball is written in Rust. Curveball uses Bevy and egui for the user interface.");
            ui.add_space(12.0);
            ui.label("Curveball is free to use. The source code is available under a permissive license. See the repository for more information.");
            ui.add_space(12.0);
            use egui::special_emojis::GITHUB;
            ui.hyperlink_to(
                format!("{GITHUB} Curveball on Github"),
                "https://github.com/MightyBurger/curveball",
            );
            ui.add_space(12.0);
            ui.label("Copyright © 2025 Jordan Johnson");
        });

        if modal.should_close() {
            local.about_open = false;
        }
    }

    // Finally, collapse all the GUI information into one curve to display on the screen.

    *curve_select = match local.selected_curve {
        SelectedCurve::CurveClassic => CurveSelect::CurveClassic(local.curveclassic_args.clone()),
        SelectedCurve::CurveSlope => CurveSelect::CurveSlope(local.curveslope_args.clone()),
        SelectedCurve::Rayto => CurveSelect::Rayto(local.rayto_args.clone()),
        SelectedCurve::Bank => CurveSelect::Bank(local.bank_args.clone()),
        SelectedCurve::Catenary => CurveSelect::Catenary(local.catenary_args.clone()),
        SelectedCurve::Serpentine => CurveSelect::Serpentine(local.serpentine_args.clone()),
        SelectedCurve::Extrusion => CurveSelect::Extrusion(ExtrusionArgs {
            profile: match local.extrusion_gui_data.selected_profile {
                SelectedProfile::Circle => {
                    ProfileSelect::Circle(local.extrusion_gui_data.profile_circle_args.clone())
                }
                SelectedProfile::Rectangle => ProfileSelect::Rectangle(
                    local.extrusion_gui_data.profile_rectangle_args.clone(),
                ),
                SelectedProfile::Annulus => {
                    ProfileSelect::Annulus(local.extrusion_gui_data.profile_annulus_args.clone())
                }
            },
            path: match local.extrusion_gui_data.selected_path {
                SelectedPath::Line => {
                    PathSelect::Line(local.extrusion_gui_data.path_line_args.clone())
                }
                SelectedPath::Revolve => {
                    PathSelect::Revolve(local.extrusion_gui_data.path_revolve_args.clone())
                }
            },
            profile_orientation: local.extrusion_gui_data.profile_orientation,
        }),
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
