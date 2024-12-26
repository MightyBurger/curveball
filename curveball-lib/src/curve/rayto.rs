use crate::brush::Brush;
use crate::curve::{Curve, CurveError};
use glam::DVec3;

use std::f64::consts::PI;

#[derive(Debug, Default, Clone)]
pub struct Rayto {
    pub n: u32,
    pub r0: f64,
    pub r1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub x: f64,
    pub y: f64,
    pub h: f64,
}

fn brush_tprism(
    rstart: f64,
    rend: f64,
    thetastart: f64,
    thetaend: f64,
    x: f64,
    y: f64,
    h: f64,
) -> Result<Brush, CurveError> {
    let x0 = rstart * (thetastart * PI / 180.0).cos();
    let y0 = rstart * (thetastart * PI / 180.0).sin();
    let x1 = rend * (thetaend * PI / 180.0).cos();
    let y1 = rend * (thetaend * PI / 180.0).sin();

    let pa = DVec3 { x, y, z: 0.0 };
    let pb = DVec3 {
        x: x0,
        y: y0,
        z: 0.0,
    };
    let pc = DVec3 {
        x: x1,
        y: y1,
        z: 0.0,
    };
    let pd = DVec3 { x, y, z: h };
    let pe = DVec3 { x: x0, y: y0, z: h };
    let pf = DVec3 { x: x1, y: y1, z: h };

    Ok(Brush::try_from_vertices(&[pa, pb, pc, pd, pe, pf], None)?)
}

impl Curve for Rayto {
    fn bake(&self) -> Result<Vec<Brush>, CurveError> {
        let n = self.n;
        let r0 = self.r0;
        let r1 = self.r1;
        let theta0 = self.theta0;
        let theta1 = self.theta1;
        let x = self.x;
        let y = self.y;
        let h = self.h;

        // get delta values
        let dr = (r1 - r0) / (n as f64);
        let dtheta = (theta1 - theta0) / (n as f64);

        let mut brushes = Vec::new();

        for i in 0..n {
            let rstart = r0 + dr * (i as f64);
            let rend = r0 + dr * (i as f64 + 1.0);
            let thetastart = theta0 + dtheta * (i as f64);
            let thetaend = theta0 + dtheta * (i as f64 + 1.0);

            brushes.push(brush_tprism(rstart, rend, thetastart, thetaend, x, y, h)?);
        }

        Ok(brushes)
    }
}
