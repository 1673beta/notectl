use base64::Engine;

struct Key {
  key: openssl::ec::EcKey<openssl::pkey::Private>,
}

impl Key {
  fn name() -> openssl::nid::Nid {
    openssl::nid::Nid::X9_62_PRIME256V1
  }

  pub fn generate() -> VapidResult<Key> {
    let group = openssl::ec::EcGroup::from_curve_name(Key::name())?;
    let key = openssl::ec::EcKey::generate(&group)?;
    Ok(Key { key: key })
  }

  pub fn to_private_raw(&self) -> String {
    let key = self.key.private_key();
    let config = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    config.encode(&key.to_vec())
  }

  pub fn to_public_raw(&self) -> String {
    let key = self.key.public_key();
    let mut ctx = openssl::bn::BigNumContext::new().unwrap();
    let group = openssl::ec::EcGroup::from_curve_name(Key::name()).unwrap();

    let keybytes = key
      .to_bytes(
        &group,
        openssl::ec::PointConversionForm::UNCOMPRESSED,
        &mut ctx,
      )
      .unwrap();
    let config = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    config.encode(&keybytes)
  }
}

pub type VapidResult<T> = std::result::Result<T, VapidError>;

#[derive(Debug)]
pub struct VapidError {
  kind: VapidErrorKind,
}

#[derive(Debug, thiserror::Error)]
pub enum VapidErrorKind {
  #[error("OpenSSL error: {:?}", .0)]
  OpenSSL(#[from] openssl::error::ErrorStack),
  #[error("JSON error: {:?}", .0)]
  Json(#[from] serde_json::Error),
  #[error("IO error: {:?}", .0)]
  File(#[from] std::io::Error),
}

impl std::fmt::Display for VapidError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.kind.fmt(f)
  }
}

impl std::error::Error for VapidError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self.kind.source()
  }
}

impl<T> From<T> for VapidError
where
  VapidErrorKind: From<T>,
{
  fn from(err: T) -> Self {
    VapidError {
      kind: VapidErrorKind::from(err),
    }
  }
}

pub struct VapidKey {
  pub private_key: String,
  pub public_key: String,
}

pub fn generate() -> Result<VapidKey, Box<dyn std::error::Error>> {
  let keypair = Key::generate().map_err(|e| e.to_string())?;
  Ok(VapidKey {
    private_key: keypair.to_private_raw(),
    public_key: keypair.to_public_raw(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_vapid_key() {
    let key = generate().unwrap();

    let private_key_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&key.private_key).expect("Failed to decode private key");
    assert_eq!(private_key_bytes.len(), 32);
    
    let public_key_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&key.public_key).expect("Failed to decode public key");
    assert_eq!(public_key_bytes.len(), 65);
    assert_eq!(public_key_bytes[0], 0x04);
  }
}