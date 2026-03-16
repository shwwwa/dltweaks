#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnabledDisabled {
    #[default]
    Disabled,
    Enabled,
}

impl EnabledDisabled {
    pub fn from_i32(value: i32) -> Self {
        if value != 0 {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            Self::Enabled => 1,
            Self::Disabled => 0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
