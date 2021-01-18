use crate::{vec_utils::*, Complex, Float, Vector3};

pub fn transfer(
    trans_pos: Vector3,
    target_pos: Vector3,
    amp: Float,
    phase: Float,
    wave_num: Float,
) -> Complex {
    let diff = sub(target_pos, trans_pos);
    let dist = norm(diff);
    amp / dist * (Complex::new(0., phase + wave_num * dist)).exp()
}
