use crate::models::object::Color;

struct ObjectVariant {
    default_col: Option<Color>,
    z_order: i32,
    force_bottom: bool,
    has_child: bool,
    has_color_child: bool,
    dont_show: bool,
}

impl ObjectVariant {
    pub fn from_id(id: u32) -> Option<ObjectVariant> {
        None  // placeholder
    }
}