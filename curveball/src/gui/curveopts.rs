// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use bevy_egui::egui;
use lib_curveball::curve::extrude;

use crate::curveargs;
// use crate::curveargs::{
//     BankArgs, CatenaryArgs, CurveClassicArgs, CurveSlopeArgs, ExtrusionArgs, RaytoArgs,
//     SerpentineArgs,
// };

pub fn curveclassic_ui(ui: &mut egui::Ui, args: &mut curveargs::CurveClassicArgs) {
    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("Start radii");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ri0).speed(0.1))
            .on_hover_text("ri0");
        ui.label("Inner radius");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ro0).speed(0.1))
            .on_hover_text("ro0");
        ui.label("Outer raidus");
    });
    ui.add_space(8.0);
    ui.label("End radii");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ri1).speed(0.1))
            .on_hover_text("ri1");
        ui.label("Inner radius");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ro1).speed(0.1))
            .on_hover_text("ro1");
        ui.label("Outer radius");
    });
    ui.add_space(8.0);
    ui.label("Angles");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.theta0).speed(0.1))
            .on_hover_text("theta0");
        ui.label("Start angle (deg)");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.theta1).speed(0.1))
            .on_hover_text("theta1");
        ui.label("End angle (deg)");
    });
    ui.add_space(8.0);
    ui.label("Heights");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.t).speed(0.1))
            .on_hover_text("t");
        ui.label("Thickness");
    });
}

pub fn curveslope_ui(ui: &mut egui::Ui, args: &mut curveargs::CurveSlopeArgs) {
    ui.checkbox(&mut args.en_const_thickness, "Force constant thickness");
    ui.horizontal(|ui| {
        ui.add_enabled_ui(args.en_const_thickness, |ui| {
            ui.add(egui::DragValue::new(&mut args.t_const_thickness).speed(0.1))
                .on_hover_text("t_const_thickness");
            ui.label("Thickness");
        });
    });
    ui.checkbox(
        &mut args.height_link_inner_outer,
        "Link inner and outer height",
    );
    ui.checkbox(&mut args.hill_link_inner_outer, "Link inner and outer hill");

    ui.separator();

    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("Start radii");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ri0).speed(0.1))
            .on_hover_text("ri0");
        ui.label("Inner radius");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ro0).speed(0.1))
            .on_hover_text("ro0");
        ui.label("Outer raidus");
    });
    ui.add_space(8.0);
    ui.label("End radii");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ri1).speed(0.1))
            .on_hover_text("ri1");
        ui.label("Inner radius");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ro1).speed(0.1))
            .on_hover_text("ro1");
        ui.label("Outer radius");
    });
    ui.add_space(8.0);
    ui.label("Angles");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.theta0).speed(0.1))
            .on_hover_text("theta0");
        ui.label("Start angle (deg)");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.theta1).speed(0.1))
            .on_hover_text("theta1");
        ui.label("End angle (deg)");
    });

    ui.separator();

    match (args.height_link_inner_outer, args.en_const_thickness) {
        (true, true) => {
            ui.label("Start height");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_0).speed(0.1))
                    .on_hover_text("height_inner_top_0");
                ui.label("Height");
            });

            ui.add_space(8.0);

            ui.label("End height");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_1).speed(0.1))
                    .on_hover_text("height_inner_top_1");
                ui.label("Height");
            });

            args.height_outer_top_0 = args.height_inner_top_0;
            args.height_inner_bot_0 = args.height_inner_top_0 - args.t_const_thickness;
            args.height_outer_bot_0 = args.height_inner_bot_0;
            args.height_outer_bot_0 = args.height_outer_top_0 - args.t_const_thickness;

            args.height_outer_top_1 = args.height_inner_top_1;
            args.height_inner_bot_1 = args.height_inner_top_1 - args.t_const_thickness;
            args.height_outer_bot_1 = args.height_inner_bot_1;
            args.height_outer_bot_1 = args.height_outer_top_1 - args.t_const_thickness;
        }
        (false, true) => {
            ui.label("Start heights");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_0).speed(0.1))
                    .on_hover_text("height_inner_top_0");
                ui.label("Inner height");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_outer_top_0).speed(0.1))
                    .on_hover_text("height_outer_top_0");
                ui.label("Outer height");
            });

            ui.add_space(8.0);

            ui.label("End heights");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_1).speed(0.1))
                    .on_hover_text("height_inner_top_1");
                ui.label("Inner height");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_outer_top_1).speed(0.1))
                    .on_hover_text("height_outer_top_1");
                ui.label("Outer height");
            });

            args.height_inner_bot_0 = args.height_inner_top_0 - args.t_const_thickness;
            args.height_outer_bot_0 = args.height_outer_top_0 - args.t_const_thickness;
            args.height_inner_bot_1 = args.height_inner_top_1 - args.t_const_thickness;
            args.height_outer_bot_1 = args.height_outer_top_1 - args.t_const_thickness;
        }

        (true, false) => {
            ui.label("Start heights");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_0).speed(0.1))
                    .on_hover_text("height_inner_top_0");
                ui.label("Height, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_bot_0).speed(0.1))
                    .on_hover_text("height_inner_bot_0");
                ui.label("Height, bottom");
            });
            args.height_outer_top_0 = args.height_inner_top_0;
            args.height_outer_bot_0 = args.height_inner_bot_0;

            ui.add_space(8.0);

            ui.label("End heights");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_1).speed(0.1))
                    .on_hover_text("height_inner_top_1");
                ui.label("Height, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_bot_1).speed(0.1))
                    .on_hover_text("height_inner_bot_1");
                ui.label("Height, bottom");
            });
            args.height_outer_top_1 = args.height_inner_top_1;
            args.height_outer_bot_1 = args.height_inner_bot_1;
        }

        (false, false) => {
            ui.label("Start heights");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_0).speed(0.1))
                    .on_hover_text("height_inner_top_0");
                ui.label("Inner height, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_bot_0).speed(0.1))
                    .on_hover_text("height_inner_bot_0");
                ui.label("Inner height, bottom");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_outer_top_0).speed(0.1))
                    .on_hover_text("height_outer_top_0");
                ui.label("Outer height, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_outer_bot_0).speed(0.1))
                    .on_hover_text("height_outer_bot_0");
                ui.label("Outer height, bottom");
            });

            ui.add_space(8.0);

            ui.label("End heights");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_top_1).speed(0.1))
                    .on_hover_text("height_inner_top_1");
                ui.label("Inner height, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_inner_bot_1).speed(0.1))
                    .on_hover_text("height_inner_bot_1");
                ui.label("Inner height, bottom");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_outer_top_1).speed(0.1))
                    .on_hover_text("height_outer_top_1");
                ui.label("Outer height, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.height_outer_bot_1).speed(0.1))
                    .on_hover_text("height_outer_bot_1");
                ui.label("Outer height, bottom");
            });
        }
    }

    ui.separator();

    match (args.hill_link_inner_outer, args.en_const_thickness) {
        (true, true) => {
            ui.label("Hill");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_inner_top).speed(0.1))
                    .on_hover_text("hill_inner_top");
                ui.label("Hill");
            });
            args.hill_outer_top = args.hill_inner_top;
            args.hill_inner_bot = args.hill_inner_top;
            args.hill_outer_bot = args.hill_outer_top;
        }
        (false, true) => {
            ui.label("Hills");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_inner_top).speed(0.1))
                    .on_hover_text("hill_inner_top");
                ui.label("Inner hill");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_outer_top).speed(0.1))
                    .on_hover_text("hill_outer_top");
                ui.label("Outer hill");
            });
            args.hill_inner_bot = args.hill_inner_top;
            args.hill_outer_bot = args.hill_outer_top;
        }
        (true, false) => {
            ui.label("Hills");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_inner_top).speed(0.1))
                    .on_hover_text("hill_inner_top");
                ui.label("Hill, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_inner_bot).speed(0.1))
                    .on_hover_text("hill_inner_bot");
                ui.label("Hill, bottom");
            });
            args.hill_outer_top = args.hill_inner_top;
            args.hill_outer_bot = args.hill_inner_bot;
        }
        (false, false) => {
            ui.label("Hills");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_inner_top).speed(0.1))
                    .on_hover_text("hill_inner_top");
                ui.label("Inner hill, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_inner_bot).speed(0.1))
                    .on_hover_text("hill_inner_bot");
                ui.label("Inner hill, bottom");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_outer_top).speed(0.1))
                    .on_hover_text("hill_outer_top");
                ui.label("Outer hill, top");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.hill_outer_bot).speed(0.1))
                    .on_hover_text("hill_outer_bot");
                ui.label("Outer hill, bottom");
            });
        }
    }
}

pub fn rayto_ui(ui: &mut egui::Ui, args: &mut curveargs::RaytoArgs) {
    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("Radii");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.r0).speed(0.1))
            .on_hover_text("r0");
        ui.label("Start radius");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.r1).speed(0.1))
            .on_hover_text("r1");
        ui.label("End radius");
    });
    ui.add_space(8.0);
    ui.label("Angles");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.theta0).speed(0.1))
            .on_hover_text("theta0");
        ui.label("Start angle (deg)");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.theta1).speed(0.1))
            .on_hover_text("theta1");
        ui.label("End angle (deg)");
    });
    ui.add_space(8.0);
    ui.label("Point location");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.x).speed(0.1))
            .on_hover_text("x");
        ui.label("x");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.y).speed(0.1))
            .on_hover_text("y");
        ui.label("y");
    });
    ui.add_space(8.0);
    ui.label("Heights");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.h).speed(0.1))
            .on_hover_text("h");
        ui.label("Thickness");
    });
}

pub fn extrusion_ui(ui: &mut egui::Ui, args: &mut curveargs::ExtrusionArgs) {
    // ui.label("Segments");

    ui.label("Profile");
    egui::ComboBox::from_id_salt("SelectedProfile")
        .selected_text(format!("{}", args.selected_profile))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut args.selected_profile,
                curveargs::SelectedProfile::Circle,
                "Circle",
            );
            ui.selectable_value(
                &mut args.selected_profile,
                curveargs::SelectedProfile::CircleSector,
                "Circle Sector",
            );
            ui.selectable_value(
                &mut args.selected_profile,
                curveargs::SelectedProfile::Rectangle,
                "Rectangle",
            );
            ui.selectable_value(
                &mut args.selected_profile,
                curveargs::SelectedProfile::Parallelogram,
                "Parallelogram",
            );
            ui.selectable_value(
                &mut args.selected_profile,
                curveargs::SelectedProfile::Annulus,
                "Annulus",
            );
            ui.selectable_value(
                &mut args.selected_profile,
                curveargs::SelectedProfile::Arbitrary,
                "Arbitrary",
            );
        });

    match args.selected_profile {
        curveargs::SelectedProfile::Circle => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_circle_args.n).speed(0.1))
                    .on_hover_text("n");
                ui.label("Resolution");
            });
            ui.add_space(8.0);
            ui.label("Size");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_circle_args.radius).speed(0.1))
                    .on_hover_text("n");
                ui.label("Radius");
            });
        }
        curveargs::SelectedProfile::CircleSector => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_circle_sector_args.n).speed(0.1))
                    .on_hover_text("n");
                ui.label("Resolution");
            });
            ui.add_space(8.0);
            ui.label("Size");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_circle_sector_args.radius).speed(0.1),
                )
                .on_hover_text("radius");
                ui.label("Radius");
            });
            ui.add_space(8.0);
            ui.label("Angles");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_circle_sector_args.start_angle)
                        .speed(0.1),
                )
                .on_hover_text("start_angle");
                ui.label("Start angle");
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_circle_sector_args.end_angle).speed(0.1),
                )
                .on_hover_text("end_angle");
                ui.label("End angle");
            });
        }
        curveargs::SelectedProfile::Rectangle => {
            ui.add_space(8.0);
            ui.label("Size");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_rectangle_args.width).speed(0.1))
                    .on_hover_text("width");
                ui.label("Width");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_rectangle_args.height).speed(0.1))
                    .on_hover_text("height");
                ui.label("Height");
            });

            ui_anchor_9point(ui, &mut args.profile_rectangle_args.anchor);
        }

        curveargs::SelectedProfile::Parallelogram => {
            ui.add_space(8.0);
            ui.label("Size");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_parallelogram_args.width).speed(0.1))
                    .on_hover_text("width");
                ui.label("Width");
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_parallelogram_args.height).speed(0.1),
                )
                .on_hover_text("height");
                ui.label("Height");
            });

            ui.add_space(8.0);
            ui.label("Offset");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_parallelogram_args.offset_x).speed(0.1),
                )
                .on_hover_text("offset_x");
                ui.label("Horizontal offset");
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_parallelogram_args.offset_z).speed(0.1),
                )
                .on_hover_text("height");
                ui.label("Vertical offset");
            });

            ui_anchor_9point(ui, &mut args.profile_parallelogram_args.anchor);
        }

        curveargs::SelectedProfile::Annulus => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_annulus_args.n).speed(0.1))
                    .on_hover_text("n");
                ui.label("Resolution");
            });
            ui.add_space(8.0);
            ui.label("Radii");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_annulus_args.inner_radius).speed(0.1),
                )
                .on_hover_text("inner_radius");
                ui.label("Inner Radius");
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut args.profile_annulus_args.outer_radius).speed(0.1),
                )
                .on_hover_text("outer_radius");
                ui.label("Outer Radius");
            });
            ui.add_space(8.0);
            ui.label("Angles");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_annulus_args.start_angle).speed(0.1))
                    .on_hover_text("start_angle");
                ui.label("Start Angle");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_annulus_args.end_angle).speed(0.1))
                    .on_hover_text("end_angle");
                ui.label("End Angle");
            });
        }
        curveargs::SelectedProfile::Arbitrary => {
            let btn_size = [20.0, 20.0];
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.add_sized(btn_size, egui::Button::new("➕")).clicked() {
                    let new_poly = vec![
                        glam::DVec2 { x: 0.0, y: 0.0 },
                        glam::DVec2 { x: 32.0, y: 0.0 },
                        glam::DVec2 { x: 32.0, y: 32.0 },
                    ];
                    args.profile_arbitrary_args.polygons.push(new_poly);
                };
                ui.label("Add polygon");
            });

            let mut polygon_to_delete: Option<usize> = None;
            for (i, polygon) in args.profile_arbitrary_args.polygons.iter_mut().enumerate() {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.add_sized(btn_size, egui::Button::new("➕")).clicked() {
                        polygon.push(glam::DVec2::default());
                    };
                    ui.label("Add point");
                });
                let mut point_to_delete: Option<usize> = None;
                for (j, point) in polygon.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.add_sized(btn_size, egui::Button::new("✖")).clicked() {
                            point_to_delete = Some(j);
                        }
                        ui.add(egui::DragValue::new(&mut point.x).speed(0.1))
                            .on_hover_text("x");
                        ui.add(egui::DragValue::new(&mut point.y).speed(0.1))
                            .on_hover_text("z");
                        ui.label(format!("Point {}", j + 1));
                    });
                }
                ui.horizontal(|ui| {
                    if ui.add_sized(btn_size, egui::Button::new("✖")).clicked() {
                        polygon_to_delete = Some(i);
                    }
                    ui.label(format!("Remove polygon {}", i + 1));
                });
                if let Some(point_to_delete) = point_to_delete {
                    polygon.remove(point_to_delete);
                }
            }
            if let Some(polygon_to_delete) = polygon_to_delete {
                args.profile_arbitrary_args
                    .polygons
                    .remove(polygon_to_delete);
            }
        }
    }

    ui.separator();

    ui.label("Path");
    egui::ComboBox::from_id_salt("PathSelect")
        .selected_text(format!("{}", args.selected_path))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut args.selected_path,
                curveargs::SelectedPath::Line,
                "Line",
            );
            ui.selectable_value(
                &mut args.selected_path,
                curveargs::SelectedPath::Revolve,
                "Revolve",
            );
            ui.selectable_value(
                &mut args.selected_path,
                curveargs::SelectedPath::Sinusoid,
                "Sinusoid",
            );
            ui.selectable_value(
                &mut args.selected_path,
                curveargs::SelectedPath::Bezier,
                "Bezier",
            );
            ui.selectable_value(
                &mut args.selected_path,
                curveargs::SelectedPath::Catenary,
                "Catenary",
            );
            ui.selectable_value(
                &mut args.selected_path,
                curveargs::SelectedPath::Serpentine,
                "Serpentine",
            );
        });

    match args.selected_path {
        curveargs::SelectedPath::Line => {
            ui.add_space(8.0);
            ui.label("Size");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_line_args.x).speed(0.1))
                    .on_hover_text("x");
                ui.label("x");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_line_args.y).speed(0.1))
                    .on_hover_text("y");
                ui.label("y");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_line_args.z).speed(0.1))
                    .on_hover_text("z");
                ui.label("z");
            });
        }
        curveargs::SelectedPath::Revolve => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_revolve_args.path_n).speed(0.1))
                    .on_hover_text("path_n");
                ui.label("Number of segments");
            });
            ui.add_space(8.0);
            ui.label("Angles");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_revolve_args.start_angle).speed(0.1))
                    .on_hover_text("path_start");
                ui.label("Starting angle");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_revolve_args.end_angle).speed(0.1))
                    .on_hover_text("path_end");
                ui.label("Ending angle");
            });
            ui.add_space(8.0);
            ui.label("Size");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_revolve_args.radius).speed(0.1))
                    .on_hover_text("r");
                ui.label("Radius");
            });
        }
        curveargs::SelectedPath::Sinusoid => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_sinusoid_args.path_n).speed(0.1))
                    .on_hover_text("path_n");
                ui.label("Number of segments");
            });
            ui.add_space(8.0);
            ui.label("Sinusoid Parameters");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_sinusoid_args.amplitude).speed(0.1))
                    .on_hover_text("r");
                ui.label("Amplitude");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_sinusoid_args.period).speed(0.1))
                    .on_hover_text("r");
                ui.label("Period");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_sinusoid_args.phase).speed(0.1))
                    .on_hover_text("r");
                ui.label("Phase");
            });
            ui.add_space(8.0);
            ui.label("Span");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_sinusoid_args.start).speed(0.1))
                    .on_hover_text("path_start");
                ui.label("Start");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_sinusoid_args.end).speed(0.1))
                    .on_hover_text("path_end");
                ui.label("End");
            });
        }
        curveargs::SelectedPath::Bezier => {
            let btn_size = [20.0, 20.0];
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_bezier_args.path_n).speed(0.1))
                    .on_hover_text("path_n");
                ui.label("Number of segments");
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.add_sized(btn_size, egui::Button::new("➕")).clicked() {
                    let last_point = args.path_bezier_args.points.last();
                    let last_point = match last_point {
                        Some(val) => *val,
                        None => glam::DVec2::default(),
                    };
                    args.path_bezier_args.points.push(last_point);
                };
                ui.label("Add point");
            });
            let mut point_to_delete: Option<usize> = None;
            for (i, point) in args.path_bezier_args.points.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if ui.add_sized(btn_size, egui::Button::new("✖")).clicked() {
                        point_to_delete = Some(i);
                    }
                    ui.add(egui::DragValue::new(&mut point.x).speed(0.1))
                        .on_hover_text("x");
                    ui.add(egui::DragValue::new(&mut point.y).speed(0.1))
                        .on_hover_text("z");
                    ui.label(format!("Point {}", i + 1));
                });
            }
            if let Some(point_to_delete) = point_to_delete {
                args.path_bezier_args.points.remove(point_to_delete);
            }
        }
        curveargs::SelectedPath::Catenary => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_catenary_args.path_n).speed(0.1))
                    .on_hover_text("path_n");
                ui.label("Number of segments");
            });
            ui.add_space(8.0);
            ui.label("Dimensions");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_catenary_args.span).speed(0.1))
                    .on_hover_text("span");
                ui.label("Span");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_catenary_args.height).speed(0.1))
                    .on_hover_text("height");
                ui.label("Height");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_catenary_args.s).speed(0.1))
                    .on_hover_text("s");
                ui.label("Length");
            });
        }
        curveargs::SelectedPath::Serpentine => {
            ui.add_space(8.0);
            ui.label("Segments");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_serpentine_args.path_n).speed(0.1))
                    .on_hover_text("path_n");
                ui.label("Number of segments");
            });
            ui.add_space(8.0);
            ui.label("End position");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_serpentine_args.x).speed(0.1))
                    .on_hover_text("x");
                ui.label("x");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_serpentine_args.z).speed(0.1))
                    .on_hover_text("z");
                ui.label("z");
            });
        }
    }

    ui.separator();
    ui.label("Profile Orientation");
    egui::ComboBox::from_id_salt("ProfileOrientation")
        .selected_text(format!("{}", args.profile_orientation))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut args.profile_orientation,
                extrude::ProfileOrientation::Constant(extrude::ProfilePlane::XZ),
                "Constant (XZ)",
            );
            ui.selectable_value(
                &mut args.profile_orientation,
                extrude::ProfileOrientation::Constant(extrude::ProfilePlane::YZ),
                "Constant (YZ)",
            );
            ui.selectable_value(
                &mut args.profile_orientation,
                extrude::ProfileOrientation::Constant(extrude::ProfilePlane::XY),
                "Constant (XY)",
            );
            ui.selectable_value(
                &mut args.profile_orientation,
                extrude::ProfileOrientation::FollowPath,
                "Follow Path",
            );
        });
}

pub fn ui_anchor_9point(ui: &mut egui::Ui, anchor: &mut extrude::profile::Anchor9Point) {
    ui.add_space(8.0);
    ui.label("Anchor position");
    ui.scope(|ui| {
        ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 3.0);
        let btn_size = [20.0, 20.0];
        let inactive_color = egui::Color32::from_rgb(0, 92, 128);
        let active_color = egui::Color32::from_rgb(0, 84, 117);
        let hovered_color = egui::Color32::from_rgb(0, 92, 128);
        ui.horizontal(|ui| {
            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::TopLeft {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("↖")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::TopLeft;
                }
            });

            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::TopCenter {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("⬆")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::TopCenter;
                }
            });

            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::TopRight {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("↗")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::TopRight;
                }
            });
        });
        ui.horizontal(|ui| {
            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::CenterLeft {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("⬅")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::CenterLeft;
                }
            });

            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::Center {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("◾")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::Center;
                }
            });

            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::CenterRight {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("➡")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::CenterRight;
                }
            });
        });
        ui.horizontal(|ui| {
            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::BottomLeft {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("↙")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::BottomLeft;
                }
            });

            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::BottomCenter {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("⬇")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::BottomCenter;
                }
            });

            ui.scope(|ui| {
                if *anchor == extrude::profile::Anchor9Point::BottomRight {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                }
                if ui.add_sized(btn_size, egui::Button::new("↘")).clicked() {
                    *anchor = extrude::profile::Anchor9Point::BottomRight;
                }
            });
        });
    });
}
