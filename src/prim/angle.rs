use crate::prelude::*;

pub const PI: f64 = std::f64::consts::PI;

pub fn deg(deg: f64) -> f64 { return deg / 180.0 * PI; }

pub fn norm(perc: f64) -> f64 { return perc * 2.0 * PI; }

pub fn atan2(a: impl Into<point::Point>, b: impl Into<point::Point>) -> f64 {
	let a = a.into();
	let b = b.into();
	return f64::atan2(b.y - a.y, b.x - a.x);
} // end fn atan2



