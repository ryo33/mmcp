use std::str::FromStr;

pub mod consts;
pub mod mcp;
pub mod port;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolVersion {
    V20241105,
    V20250326,
}

impl FromStr for ProtocolVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2024-11-05" => Self::V20241105,
            "2025-03-26" => Self::V20250326,
            _ => return Err(anyhow::anyhow!("invalid protocol version: {}", s)),
        })
    }
}

impl std::fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V20241105 => write!(f, "2024-11-05"),
            Self::V20250326 => write!(f, "2025-03-26"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_version() {
        // Test to_string
        assert_eq!(ProtocolVersion::V20241105.to_string(), "2024-11-05");
        assert_eq!(ProtocolVersion::V20250326.to_string(), "2025-03-26");

        // Test FromStr
        assert_eq!(
            "2024-11-05".parse::<ProtocolVersion>().unwrap(),
            ProtocolVersion::V20241105
        );
        assert_eq!(
            "2025-03-26".parse::<ProtocolVersion>().unwrap(),
            ProtocolVersion::V20250326
        );

        // Test error case
        assert!("invalid".parse::<ProtocolVersion>().is_err());
    }
}
