use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FGOServer {
    JP,
    NA,
}

impl FromStr for FGOServer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NA" | "na" | "Na" | "nA" => Ok(FGOServer::NA),
            "JP" | "jp" | "Jp" | "jP" => Ok(FGOServer::JP),
            _ => Err(format!("Invalid FGO server: {}", s)),
        }
    }
}

impl Display for FGOServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FGOServer::NA => write!(f, "NA"),
            FGOServer::JP => write!(f, "JP"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::server::FGOServer;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(FGOServer::from_str("NA").unwrap(), FGOServer::NA);
        assert_eq!(FGOServer::from_str("JP").unwrap(), FGOServer::JP);
        assert_eq!(FGOServer::from_str("na").unwrap(), FGOServer::NA);
        assert_eq!(FGOServer::from_str("jp").unwrap(), FGOServer::JP);
        assert_eq!(FGOServer::from_str("Na").unwrap(), FGOServer::NA);
        assert_eq!(FGOServer::from_str("Jp").unwrap(), FGOServer::JP);
        assert_eq!(FGOServer::from_str("nA").unwrap(), FGOServer::NA);
        assert_eq!(FGOServer::from_str("jP").unwrap(), FGOServer::JP);

        assert!(FGOServer::from_str("").is_err());
        assert!(FGOServer::from_str("ABC").is_err());
        assert!(FGOServer::from_str("123").is_err());
        assert!(FGOServer::from_str("NAa").is_err());
        assert!(FGOServer::from_str("JPp").is_err());
    }
}
