use bevy_egui::egui;
use bevy_egui::egui::Ui;

use crate::brush::{
    BankArgs, CatenaryArgs, CurveClassicArgs, CurveSlopeArgs, RaytoArgs, SerpentineArgs,
};

pub fn curveclassic_ui(ui: &mut Ui, args: &mut CurveClassicArgs) {
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

pub fn curveslope_ui(ui: &mut Ui, args: &mut CurveSlopeArgs) {
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

    ui.separator();

    // ui.checkbox(&mut args.height_link_start_end, "Link start and end"); // TODO: implement
    ui.checkbox(&mut args.height_link_inner_outer, "Link inner and outer");
    ui.checkbox(&mut args.height_constant_thickness, "Constant thickness");

    ui.add_space(8.0);

    match (args.height_link_inner_outer, args.height_constant_thickness) {
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

            ui.add_space(8.0);

            ui.label("Curve thickness");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.t_const_thickness).speed(0.1))
                    .on_hover_text("t_const_thickness");
                ui.label("Thickness");
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

            ui.add_space(8.0);

            ui.label("Curve thickness");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.t_const_thickness).speed(0.1))
                    .on_hover_text("t_const_thickness");
                ui.label("Thickness");
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

    ui.add_space(8.0);

    ui.separator();

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
    ui.checkbox(&mut args.hill_link_inner_outer, "Link inner and outer");
    ui.checkbox(&mut args.hill_constant_thickness, "Constant thickness");
}

pub fn rayto_ui(ui: &mut Ui, args: &mut RaytoArgs) {
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

pub fn bank_ui(ui: &mut Ui, args: &mut BankArgs) {
    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("Radii");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ri).speed(0.1))
            .on_hover_text("ri");
        ui.label("Inner radius");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.ro).speed(0.1))
            .on_hover_text("ro");
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
        ui.add(egui::DragValue::new(&mut args.h).speed(0.1))
            .on_hover_text("h");
        ui.label("Height");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.t).speed(0.1))
            .on_hover_text("t");
        ui.label("Thickness");
    });
    ui.horizontal(|ui| {
        ui.checkbox(&mut args.fill, "Filled").on_hover_text("fill");
    });
}

pub fn catenary_ui(ui: &mut Ui, args: &mut CatenaryArgs) {
    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("Start position");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.x0).speed(0.1))
            .on_hover_text("x0");
        ui.label("x");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.z0).speed(0.1))
            .on_hover_text("z0");
        ui.label("z");
    });
    ui.add_space(8.0);
    ui.label("End position");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.x1).speed(0.1))
            .on_hover_text("x1");
        ui.label("x");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.z1).speed(0.1))
            .on_hover_text("z1");
        ui.label("z");
    });
    ui.add_space(8.0);
    ui.label("Dimensions");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.s).speed(0.1))
            .on_hover_text("s");
        ui.label("Length");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.w).speed(0.1))
            .on_hover_text("w");
        ui.label("Width");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.t).speed(0.1))
            .on_hover_text("t");
        ui.label("Thickness");
    });
    // TODO: allow user to change initial guess
}

pub fn serpentine_ui(ui: &mut Ui, args: &mut SerpentineArgs) {
    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("End position");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.x).speed(0.1))
            .on_hover_text("x");
        ui.label("x");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.z).speed(0.1))
            .on_hover_text("z");
        ui.label("z");
    });
    ui.add_space(8.0);
    ui.label("Dimensions");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.w).speed(0.1))
            .on_hover_text("w");
        ui.label("Width");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.t).speed(0.1))
            .on_hover_text("t");
        ui.label("Thickness");
    });
}
