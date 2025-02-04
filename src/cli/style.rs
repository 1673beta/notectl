use anstyle::AnsiColor;
use anstyle::Effects;

pub fn style() -> clap::builder::Styles {
  clap::builder::Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .error(
      AnsiColor::Red
        .on_default()
        .effects(Effects::BOLD)
        .effects(Effects::DOUBLE_UNDERLINE),
    )
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(
      AnsiColor::White
        .on_default()
        .effects(Effects::DOUBLE_UNDERLINE),
    )
    .valid(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::Red.on_default().effects(Effects::BOLD))
}
