use iced::Task;

#[derive(Debug, Clone, Default)]
pub struct A {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Static,
    Loading(String),
    Loaded(String),
}

impl A {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Static => Task::none(),
            Message::Loading(input) => {
                self.content = input.to_string();
                Task::perform(Self::ac_search(input), Message::Loaded)
            }
            Message::Loaded(res) => {
                println!("{res}");
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        iced::widget::TextInput::new("Search", "a")
            .on_input(|x| Message::Static)
            // .on_input_maybe(Some(Message::Loading))
            .into()
    }

    async fn ac_search(input: String) -> String {
        let base_req = String::from("https://ac.rule34.xxx/autocomplete.php?q=");
        let res = match reqwest::get(&format!("{}{}", base_req, input)).await {
            Ok(res) => res,
            Err(e) => panic!("{e}"),
        };
        match res.text().await {
            Ok(res) => res,
            Err(e) => panic!("{e}"),
        }
    }
}
