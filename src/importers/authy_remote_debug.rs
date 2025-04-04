/*

Import from JSON file exported from a script executed from remote debugging.
For more information see https://gist.github.com/gboudreau/94bb0c11a6209c82418d01a59d958c93

*/

use serde::Deserialize;
use serde_json;
use std::fs::read_to_string;

use crate::otp::otp_element::OTPElement;

#[derive(Deserialize)]
struct AuthyExportedJsonElement {
    name: String,
    secret: String,
    uri: String,
}

impl AuthyExportedJsonElement {
    pub fn get_type(&self) -> String {
        let default_value = "totp";
        let args: Vec<&str> = self.uri.split('/').collect();
        String::from(*args.get(2).unwrap_or(&default_value))
    }

    pub fn get_digits(&self) -> u64 {
        let default_value = 6;
        let args: Vec<&str> = self.uri.split('/').collect();
        match args.get(3) {
            Some(s) => {
                let args: Vec<&str> = s.split('?').collect();
                match args.get(1) {
                    Some(s) => {
                        let args: Vec<&str> =
                            s.split('&').filter(|s| s.starts_with("digits=")).collect();
                        match args.get(0) {
                            Some(s) => s.parse::<u64>().unwrap_or(default_value),
                            None => default_value,
                        }
                    }
                    None => default_value,
                }
            }
            None => default_value,
        }
    }

    pub fn get_issuer(&self) -> String {
        let default_value = "";
        let args: Vec<&str> = self.uri.split('/').collect();
        match args.get(3) {
            Some(s) => {
                let args: Vec<&str> = s.split('?').collect();
                let issuer = args.get(0).unwrap_or(&default_value);
                match urlencoding::decode(issuer) {
                    Ok(r) => r.into_owned(),
                    Err(_e) => issuer.to_string(),
                }
            }
            None => String::from(default_value),
        }
    }
}

pub fn import(file_path: &str) -> Result<Vec<OTPElement>, String> {
    let json = match read_to_string(file_path) {
        Ok(r) => r,
        Err(e) => return Err(format!("{:?}", e)),
    };
    let elements: Vec<AuthyExportedJsonElement> = match serde_json::from_str(json.as_str()) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error during deserializing: {:?}", e)),
    };

    Ok(elements
        .into_iter()
        .map(|e| {
            let type_ = e.get_type();
            let digits = e.get_digits();
            OTPElement::new(
                e.secret.to_uppercase().replace('=', ""),
                e.get_issuer(),
                e.name,
                digits,
                type_,
                "SHA1".to_string(),
                "".to_string(),
                0,
                0,
                30,
                0,
                Vec::new(),
            )
        })
        .collect())
}
