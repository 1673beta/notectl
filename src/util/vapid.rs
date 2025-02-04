use vapid::Key;

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
