use std::sync::{Arc, RwLock};

type Point = (f32, f32, f32);

lazy_static::lazy_static! {
  static ref VIEWER_POSITION: RwLock<(f32,f32,f32)> = RwLock::new((0.0, 0.0, 0.0));
  static ref VIEWER_ANGLE: RwLock<f32> = RwLock::new(std::f32::consts::PI * 0.5);
  static ref VIEWER_Y_SHIFY: RwLock<f32> = RwLock::new(0.0);
  static ref DIRTY_MARK: RwLock<bool> = RwLock::new(true);
}

pub fn move_viewer_by(p: Point) {
  let p1 = to_viewer_axis(p);
  let p0 = *VIEWER_POSITION.write().unwrap();
  let p2 = (p1.0 + p0.0, p1.1 + p0.1, p1.2 + p0.2);
  *VIEWER_POSITION.write().unwrap() = p2;
  mark_dirty();
}

pub fn rotate_view_by(x: f32) {
  let mut angle = VIEWER_ANGLE.write().unwrap();
  *angle += x;
  mark_dirty();
}

pub fn shift_viewer_by(dy: f32) {
  let mut y_shift = VIEWER_Y_SHIFY.write().unwrap();
  *y_shift += 2.0 * dy;
  mark_dirty();
}

/// compare the point to viewer's position and angle
pub fn to_viewer_axis(p: Point) -> Point {
  let half_pi: f32 = std::f32::consts::PI * 0.5;

  // let length = (p.0 * p.0 + p.1 * p.1 + p.2 * p.2).sqrt();
  let angle: f32 = *VIEWER_ANGLE.read().unwrap();
  let project_distance = 20.0;
  let y_shift = *VIEWER_Y_SHIFY.read().unwrap();
  // vertical angle
  let v_angle = (y_shift / project_distance).atan();

  let from_x = (p.0 * (angle - half_pi).cos(), 0.0, -1. * p.0 * (angle - half_pi).sin());

  let from_y = (
    p.1 * (v_angle + half_pi).cos() * angle.cos(),
    p.1 * (v_angle + half_pi).sin(),
    -1. * p.1 * (v_angle + half_pi).cos() * angle.sin(),
  );

  let from_z = (
    p.2 * -1. * v_angle.cos() * angle.cos(),
    p.2 * -1. * v_angle.sin(),
    p.2 * v_angle.cos() * angle.sin(),
  );

  (
    from_x.0 + from_y.0 + from_z.0,
    from_x.1 + from_y.1 + from_z.1,
    from_x.2 + from_y.2 + from_z.2,
  )
}

/// get a vector at viewing position at length 600
pub fn new_lookat_point() -> Point {
  let p: Point = (
    VIEWER_ANGLE.read().unwrap().cos() * 400.,
    *VIEWER_Y_SHIFY.read().unwrap() * 20.,
    VIEWER_ANGLE.read().unwrap().sin() * -400.,
  );

  let l = (p.0 * p.0 + p.1 * p.1 + p.2 * p.2).sqrt();
  let ratio = 600. / l;
  (p.0 * ratio, p.1 * ratio, p.2 * ratio)
}

pub fn get_position() -> Point {
  *VIEWER_POSITION.read().unwrap()
}

pub fn mark_dirty() {
  *DIRTY_MARK.write().unwrap() = true;
  // web_sys::console::log_1(&"dirty".into());
}

pub fn requested_rendering() -> bool {
  let ret = *DIRTY_MARK.write().unwrap();
  *DIRTY_MARK.write().unwrap() = false;
  ret
}

pub fn get_shift_y() -> f32 {
  *VIEWER_Y_SHIFY.read().unwrap()
}

pub fn reset_shift_y() {
  *VIEWER_Y_SHIFY.write().unwrap() = 0.0;
  mark_dirty()
}
