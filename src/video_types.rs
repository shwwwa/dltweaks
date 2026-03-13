/** Corresponds to texture quality in-game. */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureQuality {
    Low,
    Medium,
    #[default]
    High,
}

/** Corresponds to shadow map size in-game. */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowQuality {
    /** 512 */
    Low,
    /** 1024 */
    Medium,
    #[default]
    /** 2048 */
    High,
    /** 4096 */
    VeryHigh,
    /** Custom value input by user. */
    Custom,
}

/** Corresponds to foliage quality in-game. */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoliageQuality {
    /** 2 */
    Low,
    /** 1 */
    Medium,
    #[default]
    /** 0 */
    High,
    /** Custom value input by user. */
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaxFpsPreset {
    #[default]
    Uncapped,
    Fps30,
    Fps60,
    Fps80,
    Fps100,
    Fps120,
    Fps144,
    Custom,
}

impl TextureQuality {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "low" => Self::Low,
            "medium" => Self::Medium,
            "high" => Self::High,
            _ => Self::High,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }
}

impl ShadowQuality {
    pub fn from_values(map_size: u32, spot_size: u32) -> Self {
        match (map_size, spot_size) {
            (512, 512) => Self::Low,
            (1024, 1024) => Self::Medium,
            (2048, 2048) => Self::High,
            (4096, 4096) => Self::VeryHigh,
            _ => Self::Custom,
        }
    }

    pub fn map_size(&self) -> u32 {
        match self {
            Self::Low => 512,
            Self::Medium => 1024,
            Self::High => 2048,
            Self::VeryHigh => 4096,
            Self::Custom => 2048, // fallback
        }
    }

    pub fn spot_size(&self) -> u32 {
        self.map_size()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::VeryHigh => "Very High",
            Self::Custom => "Custom",
        }
    }
}

impl FoliageQuality {
    pub fn from_value(value: i32) -> Self {
        match value {
            2 => Self::Low,
            1 => Self::Medium,
            0 => Self::High,
            _ => Self::Custom,
        }
    }

    pub fn as_value(&self) -> i32 {
        match self {
            Self::Low => 2,
            Self::Medium => 1,
            Self::High => 0,
            Self::Custom => 0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Custom => "Custom",
        }
    }
}

impl MaxFpsPreset {
    pub fn from_value(value: i32) -> Self {
        match value {
            0 | _ if value < 0 => Self::Uncapped,
            30 => Self::Fps30,
            60 => Self::Fps60,
            80 => Self::Fps80,
            100 => Self::Fps100,
            120 => Self::Fps120,
            144 => Self::Fps144,
            _ => Self::Custom,
        }
    }

    pub fn as_value(&self) -> i32 {
        match self {
            Self::Uncapped => 0,
            Self::Fps30 => 30,
            Self::Fps60 => 60,
            Self::Fps80 => 80,
            Self::Fps100 => 100,
            Self::Fps120 => 120,
            Self::Fps144 => 144,
            Self::Custom => 0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uncapped => "Uncapped",
            Self::Fps30 => "30",
            Self::Fps60 => "60",
            Self::Fps80 => "80",
            Self::Fps100 => "100",
            Self::Fps120 => "120",
            Self::Fps144 => "144",
            Self::Custom => "Custom",
        }
    }
}
