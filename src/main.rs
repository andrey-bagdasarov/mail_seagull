use iced::{Sandbox, Settings};
use crate::mail_seagull::MailSeagull;

mod ui;
mod mail_seagull;

fn main() -> iced::Result {
    MailSeagull::run(Settings::default())
}
