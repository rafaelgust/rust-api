use std::str;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use data_encoding::BASE32HEX_NOPAD;

#[cfg(test)]
pub fn string_to_uuid(uuid: &str) -> Uuid {
    Uuid::parse_str(&uuid).unwrap()
}

pub fn uuid_to_base32hex(uuid: Uuid) -> String {
    BASE32HEX_NOPAD.encode(uuid.as_bytes())
}

pub fn base32hex_to_uuid(base32hex: &str) -> Result<Uuid, String> {
    let decoded_bytes = match BASE32HEX_NOPAD.decode(base32hex.as_bytes()) {
        Ok(bytes) => bytes,
        Err(err) => return Err(format!("Error decoding: {}", err)),
    };

    match Uuid::from_slice(&decoded_bytes) {
        Ok(uuid) => Ok(uuid),
        Err(err) => Err(format!("Error creating UUID from decoded bytes: {}", err)),
    }
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// testes unitários
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_uuid() {
        let uuid_str = "018e15ba-ff46-7023-540b-bffb6d3518e4";
        let uuid = string_to_uuid(uuid_str);
        let expected_uuid = Uuid::parse_str("018e15ba-ff46-7023-540b-bffb6d3518e4").unwrap();
        assert_eq!(uuid, expected_uuid);
    }

    #[test]
    fn test_uuid_to_base32hex() {
        let uuid_str = "018e15ba-ff46-7023-540b-bffb6d3518e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        let base32hex = uuid_to_base32hex(uuid);
        assert_eq!(base32hex, "0671BENV8PO26L0BNVTMQD8OSG");
    }

    #[test]
    fn test_base32hex_to_uuid() {
        let base32hex_str = "0671BENV8PO26L0BNVTMQD8OSG";
        let uuid = base32hex_to_uuid(&base32hex_str).unwrap();
        let expected_uuid = Uuid::parse_str("018e15ba-ff46-7023-540b-bffb6d3518e4").unwrap();
        assert_eq!(uuid, expected_uuid);
    }

    #[test]
    fn test_base32hex_to_uuid_invalid() {
        let base32hex_str = "0671BENV8PO26L0BNV";  // string base32hex inválida (encurtada)
        let result = base32hex_to_uuid(&base32hex_str);
        assert!(result.is_err());
    }
}