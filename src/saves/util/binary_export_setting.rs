#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum BinaryExportSetting {
    None,
    IncludeFooter = 1,
    IncludeHeader = 1 << 1,
}

impl BinaryExportSetting {
    pub fn has_flag_fast(&self, settings: BinaryExportSetting) -> bool {
        *self as u8 & settings as u8 != 0
    }
}
