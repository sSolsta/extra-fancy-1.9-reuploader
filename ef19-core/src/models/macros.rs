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
        {
            let key = $i;
            let val = $m.remove(key).ok_or_else(|| KeyError::Missing{key: key.to_string()})?;
            match val.parse::<u8>()
                .map_err(|_| KeyError::Invalid{key: key.to_string(), val: val.to_string()})? {
                0u8 => Ok(false),
                1u8 => Ok(true),
                _ => Err(KeyError::Invalid{key: key.to_string(), val: val.to_string()}),
            }?
        }
        /* match $m.remove($i)?.parse::<u8>().ok()? {
            0u8 => false,
            1u8 => true,
            _ => { return None; },
        } */
    };
    ($m:expr, $i:expr, String) => {
        {
            let key = $i;
            $m.remove(key).ok_or(Error::from(KeyError::Missing{key: key.to_string()}))?
        }
    };
    ($m:expr, $i:expr, $t:ty) => {
        {
            let key = $i;
            let val = $m.remove(key).ok_or_else(|| KeyError::Missing{key: key.to_string()})?;
            val.parse::<$t>().map_err(|_| KeyError::Invalid{key: key.to_string(), val: val.to_string()})?
        }
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

pub(crate) use attr_from_map;

macro_rules! attr_from_map_real {
    ($m:expr, $i:expr, bool) => {
        
    };
    ($m:expr, $i:expr, String) => {
        let key = $i;
        $m.remove(key).ok_or_else(Error::from(KeyError::Missing{key}))
    };
    ($m:expr, $i:expr, $t:ty) => {
        
    };
}