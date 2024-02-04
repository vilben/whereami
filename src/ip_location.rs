use std::fmt::{Display, Formatter, Pointer};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpLocation {
    pub country: String,
    pub city: String,
    pub lat: f64,
    pub lon: f64,
}

impl Display for IpLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.city)
    }
}
