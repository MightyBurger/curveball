use crate::curve::{Curve, CurveResult, MAX_HULL_ITER};
use crate::map::Brush;
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

impl Curve for Rayto {
    fn bake(&self) -> CurveResult<Vec<Brush>> {
        // get delta values
        let dr = (self.r1 - self.r0) / (self.n as f64);
        let dtheta = (self.theta1 - self.theta0) / (self.n as f64);

        let mut brushes = Vec::new();

        for i in 0..self.n {
            let rstart = self.r0 + dr * (i as f64);
            let rend = self.r0 + dr * (i as f64 + 1.0);
            let thetastart = self.theta0 + dtheta * (i as f64);
            let thetaend = self.theta0 + dtheta * (i as f64 + 1.0);

            let x0 = rstart * (thetastart * PI / 180.0).cos();
            let y0 = rstart * (thetastart * PI / 180.0).sin();
            let x1 = rend * (thetaend * PI / 180.0).cos();
            let y1 = rend * (thetaend * PI / 180.0).sin();

            let pa = DVec3 {
                x: self.x,
                y: self.y,
                z: 0.0,
            };
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
            let pd = DVec3 {
                x: self.x,
                y: self.y,
                z: self.h,
            };
            let pe = DVec3 {
                x: x0,
                y: y0,
                z: self.h,
            };
            let pf = DVec3 {
                x: x1,
                y: y1,
                z: self.h,
            };

            brushes.push(Brush::try_from_vertices(
                &vec![pa, pb, pc, pd, pe, pf],
                MAX_HULL_ITER,
            )?);
        }

        Ok(brushes)
    }
}
