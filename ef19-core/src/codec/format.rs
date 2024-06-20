use crate::models::object::Color;

pub trait GdFormat {
    fn gd_format(self) -> String;
}

impl GdFormat for bool {
    fn gd_format(self) -> String {
        if self { "1" } else { "0" }.to_string()
    }
}

impl GdFormat for f32 {
    fn gd_format(self) -> String {
        let string = format!("{:.4}", self);
        let trimmed = string.trim_end_matches("0");
        
        match trimmed.strip_suffix(".") {
            Some(s) => s,
            None => trimmed,
        }.to_string()
    }
}

impl GdFormat for Color {
    fn gd_format(self) -> String {
        (self as isize).to_string()
    }
}

impl GdFormat for String {
    fn gd_format(self) -> String {
        self
    }
}

macro_rules! fmt_int {
    ($t:ty) => {
        impl GdFormat for $t {
            fn gd_format(self) -> String {
                self.to_string()
            }
        }
    }
}
fmt_int!(i8);
fmt_int!(u8);
fmt_int!(i16);
fmt_int!(u16);
fmt_int!(i32);
fmt_int!(u32);
