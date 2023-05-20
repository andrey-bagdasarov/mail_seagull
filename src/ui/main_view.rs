use iced::{Element, Length, Sandbox};
use iced::widget::{Column, Container};
use crate::mail_seagull::MailSeagull;

#[derive(Debug, Clone, Copy)]
pub enum MainViewMessage {
    GetMail
}

impl Sandbox for MailSeagull {
    type Message = MainViewMessage;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Mail Seagull")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MainViewMessage::GetMail => ()
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(col)
            .into()
    }
}
