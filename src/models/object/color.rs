pub enum Color {
    Player1 = 1,
    Player2 = 2,
    Col1 = 3,
    Col2 = 4,
    LightBG = 5,
    Col3 = 6,
    Col4 = 7,
    DLine = 8,  // 3d line
}
impl Color {
    pub fn from_old_id(id: u32) -> Option<Color> {
        match id {
            1 => Some(Color::Player1),
            2 => Some(Color::Player2),
            3 => Some(Color::Col1),
            4 => Some(Color::Col2),
            5 => Some(Color::LightBG),
            6 => Some(Color::Col3),
            7 => Some(Color::Col4),
            8 => Some(Color::DLine),
            _ => None,
        }
    }
    pub fn from_new_id(id: u32) -> Option<Color> {
        match id {
            1 => Some(Color::Col1),
            2 => Some(Color::Col2),
            3 => Some(Color::Col3),
            4 => Some(Color::Col4),
            5 => Some(Color::DLine),  // for those who may be using color 5 for 3dl
            1003 => Some(Color::DLine),
            1005 => Some(Color::Player1),
            1006 => Some(Color::Player2),
            1007 => Some(Color::LightBG),
            _ => None,
        }
    }
}