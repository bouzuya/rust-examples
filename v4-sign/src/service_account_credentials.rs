use crate::{Error, ErrorKind};

pub struct ServiceAccountCredentials {
    pub client_email: String,
    pub private_key: String,
}

impl ServiceAccountCredentials {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        let mut file = std::fs::File::open(path).map_err(ErrorKind::File)?;
        let mut s = String::new();
        std::io::Read::read_to_string(&mut file, &mut s).map_err(ErrorKind::File)?;
        let json_value: serde_json::Value =
            serde_json::from_str(s.as_str()).map_err(ErrorKind::InvalidServiceAccountJson)?;
        let json_object = json_value
            .as_object()
            .ok_or_else(|| ErrorKind::ServiceAccountJsonRootIsNotObject)?;
        let client_email = json_object
            .get("client_email")
            .ok_or_else(|| ErrorKind::ServiceAccountJsonClientEmailIsNotFound)?
            .as_str()
            .ok_or_else(|| ErrorKind::ServiceAccountJsonClientEmailIsNotString)?
            .to_string();
        let private_key = json_object
            .get("private_key")
            .ok_or_else(|| ErrorKind::ServiceAccountJsonPrivateKeyIsNotFound)?
            .as_str()
            .ok_or_else(|| ErrorKind::ServiceAccountJsonPrivateKeyIsNotString)?
            .to_string();
        Ok(Self {
            client_email,
            private_key,
        })
    }
}
