use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(version, author, about)]
#[command(styles(Styles::styled()
    .usage(AnsiColor::Magenta.on_default()  | Effects::BOLD)
    .header(AnsiColor::Magenta.on_default() | Effects::BOLD)))]
pub struct Options {
    /// The MAC address of the device that needs to be waken up.
    pub mac_address: String,
}
