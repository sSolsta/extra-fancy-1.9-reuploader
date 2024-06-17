macro_rules! attr_from_map {
    ($m:expr, $i:expr, Option<bool>) => {
        match $m.remove($i) {
            Some(v) => match v.parse::<u8>() {
                Ok(0u8) => Some(false),
                Ok(1u8) => Some(true),
                _ => None,
            },
            None => None,
        }
    };
    ($m:expr, $i:expr, Option<String>) => {
        $m.remove($i)
    };
    ($m:expr, $i:expr, Option<$t:ty>) => {
        match $m.remove($i) {
            Some(v) => v.parse::<$t>().ok(),
            None => None,
        }
    };
    ($m:expr, $i:expr, bool) => {
        match $m.remove($i)?.parse::<u8>().ok()? {
            0u8 => false,
            1u8 => true,
            _ => { return None; },
        }
    };
    ($m:expr, $i:expr, String) => {
        $m.remove($i)?
    };
    ($m:expr, $i:expr, $t:ty) => {
        $m.remove($i)?.parse::<$t>().ok()?
    };
    ($m:expr, $i:expr, bool, default = $d:expr) => {
        match $m.remove($i) {
            Some(v) => match v.parse::<u8>() {
                Ok(0u8) => false,
                Ok(1u8) => true,
                _ => $d,
            },
            None => $d,
        }
    };
    ($m:expr, $i:expr, String, default = $d:expr) => {
        match $m.remove($i) {
            Some(v) => v,
            None => $d,
        }
    };
    ($m:expr, $i:expr, $t:ty, default = $d:expr) => {
        match $m.remove($i) {
            Some(v) => match v.parse::<$t>() {
                Ok(v) => v,
                Err(_) => $d,
            },
            None => $d,
        }
    };
}

macro_rules! attr_from_gmd {
    ($m:expr, $i:expr, $t:ty) => {
        panic!("Not implemented")
    };
    ($m:expr, $i:expr, $t:ty, default = $d:expr) => {
        panic!("Not implemented")
    };
}

pub(crate) use attr_from_map;