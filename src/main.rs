use iced::{keyboard::Key, Sandbox, Element, Settings,};
use iced::widget::{column, text, text_input, Column, TextInput};
mod keygen;

struct KeyGen {
    username: String,
    key: Option<String>,

}
#[derive(Debug, Clone)]
pub enum Message{
    UsernameUpdate(String),

}
impl Sandbox for KeyGen {
    type Message = Message;
    fn new() -> Self {
        KeyGen { username: "".to_string(), key: None }

    }
    fn title(&self) -> String {
        "Lain Iwakura's KeyGen".to_string()
    }
    fn update(&mut self, message : Self::Message) {
        match message {
            Message::UsernameUpdate(username) => {
                self.username = username;
                self.key = Some(keygen::generatePassword(self.username.clone()));

            }
        }
    }
    fn view(&self) -> Element<Self::Message> {
        if self.key.is_some(){
        column([text_input("username", &self.username).on_input(Message::UsernameUpdate).into(),
        text_input("", &self.key.clone().unwrap()).into()]).into()
        } else {
            column([text_input("username", &self.username).on_input(Message::UsernameUpdate).into()]).into()
        }
    }

}
fn main() {
    KeyGen::run(Settings::default());
}
