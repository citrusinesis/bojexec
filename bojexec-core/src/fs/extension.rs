#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeExtension {
    Rust,
    Go,
    Python,
    Cpp,
    C,
    Java,
    Unknown,
}

impl CodeExtension {
    pub fn from_ext_str(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "go" => Self::Go,
            "py" => Self::Python,
            "cpp" | "cxx" | "cc" => Self::Cpp,
            "c" => Self::C,
            "java" => Self::Java,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Go => "go",
            Self::Python => "py",
            Self::Cpp => "cpp",
            Self::C => "c",
            Self::Java => "java",
            Self::Unknown => "",
        }
    }
}
