pub fn gen() -> Result<(), Box<dyn std::error::Error>> {
  let key = crate::util::vapid::generate()?;
  let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
  println!("{style}Please copy the following keys and paste them into Service Worker Settings in control panel.{style:#}");
  println!("Private Key: {}", key.private_key);
  println!("Public Key: {}", key.public_key);
  Ok(())
}
