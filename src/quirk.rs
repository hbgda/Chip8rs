#[derive(Clone, Copy, Debug)]
pub struct Quirk {
    pub vf_reset: bool,
    pub mem_inc: bool,
    pub display_wait: bool,
    pub clipping: bool,
    pub shift_x: bool,
    pub jump_vx: bool
}