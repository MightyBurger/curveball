use crate::brush::PathSelect;
use crate::brush::ProfileSelect;
use bevy_egui::egui;
use curveball_lib::curve::extrude;

use crate::brush::{
    BankArgs, CatenaryArgs, CurveClassicArgs, CurveSlopeArgs, ExtrusionArgs, RaytoArgs,
    SerpentineArgs,
};

pub fn curveclassic_ui(ui: &mut egui::Ui, args: &mut CurveClassicArgs) {
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

pub fn curveslope_ui(ui: &mut egui::Ui, args: &mut CurveSlopeArgs) {
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

pub fn rayto_ui(ui: &mut egui::Ui, args: &mut RaytoArgs) {
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

pub fn bank_ui(ui: &mut egui::Ui, args: &mut BankArgs) {
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

pub fn catenary_ui(ui: &mut egui::Ui, args: &mut CatenaryArgs) {
    ui.label("Segments");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.n).speed(0.1))
            .on_hover_text("n");
        ui.label("Number of segments");
    });
    ui.add_space(8.0);
    ui.label("Dimensions");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.span).speed(0.1))
            .on_hover_text("span");
        ui.label("Span");
    });
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut args.height).speed(0.1))
            .on_hover_text("height");
        ui.label("Height");
    });
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

pub fn serpentine_ui(ui: &mut egui::Ui, args: &mut SerpentineArgs) {
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

pub fn extrusion_ui(ui: &mut egui::Ui, args: &mut ExtrusionArgs) {
    // ui.label("Segments");

    ui.label("Profile");
    egui::ComboBox::from_id_salt("ProfileSelect")
        .selected_text(format!("{:?}", args.profile))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut args.profile, ProfileSelect::Circle, "Circle");
            ui.selectable_value(&mut args.profile, ProfileSelect::Rectangle, "Rectangle");
        });

    ui.add_space(8.0);
    match args.profile {
        ProfileSelect::Circle => {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_circle_args.n).speed(0.1))
                    .on_hover_text("n");
                ui.label("Resolution");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.profile_circle_args.radius).speed(0.1))
                    .on_hover_text("n");
                ui.label("Radius");
            });
        }
        ProfileSelect::Rectangle => {
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

            ui.label("Anchor position");

            ui.scope(|ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 3.0);
                let btn_size = [20.0, 20.0];
                let inactive_color = egui::Color32::from_rgb(0, 92, 128);
                let active_color = egui::Color32::from_rgb(0, 84, 117);
                let hovered_color = egui::Color32::from_rgb(0, 92, 128);
                ui.horizontal(|ui| {
                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::TopLeft
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("↖")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::TopLeft;
                        }
                    });

                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::TopCenter
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("⬆")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::TopCenter;
                        }
                    });

                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::TopRight
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("↗")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::TopRight;
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::CenterLeft
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("⬅")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::CenterLeft;
                        }
                    });

                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::Center
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("◾")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::Center;
                        }
                    });

                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::CenterRight
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("➡")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::CenterRight;
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::BottomLeft
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("↙")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::BottomLeft;
                        }
                    });

                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::BottomCenter
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("⬇")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::BottomCenter;
                        }
                    });

                    ui.scope(|ui| {
                        if args.profile_rectangle_args.anchor
                            == extrude::profile::RectangleAnchor::BottomRight
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = inactive_color;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill = active_color;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hovered_color;
                        }
                        if ui.add_sized(btn_size, egui::Button::new("↘")).clicked() {
                            args.profile_rectangle_args.anchor =
                                extrude::profile::RectangleAnchor::BottomRight;
                        }
                    });
                });
            });
        }
    }

    ui.separator();

    ui.label("Path");
    egui::ComboBox::from_id_salt("PathSelect")
        .selected_text(format!("{:?}", args.path))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut args.path, PathSelect::Revolve, "Revolve");
        });

    ui.add_space(8.0);
    match args.path {
        PathSelect::Revolve => {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_n).speed(0.1))
                    .on_hover_text("path_n");
                ui.label("Segments");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_start).speed(0.1))
                    .on_hover_text("path_start");
                ui.label("Starting angle");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_end).speed(0.1))
                    .on_hover_text("path_end");
                ui.label("Ending angle");
            });
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut args.path_revolve_args.radius).speed(0.1))
                    .on_hover_text("r");
                ui.label("Radius");
            });
        }
    }

    ui.separator();
    ui.label("Profile Orientation");
    egui::ComboBox::from_id_salt("ProfileOrientation")
        .selected_text(format!("{:?}", args.profile_orientation))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut args.profile_orientation,
                extrude::ProfileOrientation::Constant,
                "Constant",
            );
            ui.selectable_value(
                &mut args.profile_orientation,
                extrude::ProfileOrientation::FollowPath,
                "Follow Path",
            );
        });
}
