use std::{fs, path};

use autocomplete::{SearchResult, TagType};
use iced::{advanced::Widget, Element, Point, Task, Theme};

#[allow(unused)]
mod autocomplete;
mod cstm_widgets;
mod download;
#[allow(unused)]
mod testt;

fn main() -> iced::Result {
    //std::process::exit(1);
    //iced::run("", testt::A::update, testt::A::view)
    iced::application("R34 Viewer", Viewer::update, Viewer::view)
        .subscription(keys)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Last,
    DeleteCurrent,
    Quit,
    ChangeState(State),
    SearchInput(String),
    Searched(String),
    AddTag,
    Download,
}

#[allow(unused)]
#[derive(Debug, Clone)]
enum State {
    Viewer,
    Download,
}

#[derive(Debug, Clone)]
struct Viewer {
    img_paths: Vec<String>,
    current_img: usize,
    search_results: Vec<SearchResult>,
    search_bar_content: String,
    state: State,
    tags: Vec<String>,
}

impl Default for Viewer {
    fn default() -> Self {
        let img_paths = get_paths();
        Self {
            img_paths,
            current_img: 0,
            search_results: Vec::new(),
            search_bar_content: String::new(),
            state: State::Download,
            tags: Vec::new(),
        }
    }
}

impl Viewer {
    fn update(&mut self, message: Message) -> Task<Message> {
        if !self.img_paths.is_empty()
            && !path::Path::new(&self.img_paths[self.current_img]).exists()
        {
            self.delete_img(self.current_img);
        }

        match message {
            Message::Next => {
                if self.img_paths.len() - 1 > self.current_img {
                    self.current_img += 1;
                } else {
                    self.current_img = 0;
                }

                Task::none()
            }
            Message::Last => {
                if self.current_img > 0 {
                    self.current_img -= 1;
                } else {
                    self.current_img = self.img_paths.len() - 1;
                }

                Task::none()
            }
            Message::DeleteCurrent => {
                self.delete_img(self.current_img);
                Task::none()
            }
            Message::Quit => iced::exit(),
            Message::SearchInput(input) => {
                self.search_bar_content = input.clone();
                Task::perform(autocomplete::ac_search(input), Message::Searched)
            }
            Message::Searched(response) => {
                self.search_results = SearchResult::parse(&response);
                Task::none()
            }
            Message::ChangeState(s) => {
                if let State::Viewer = s {
                    self.update_paths();
                }
                self.state = s;
                Task::none()
            }
            Message::AddTag => {
                for t in self.search_results.clone() {
                    if self.search_bar_content == t.value
                        || self.search_bar_content == "-".to_string() + &t.value.clone()
                    {
                        self.tags.push(self.search_bar_content.clone());
                        self.search_bar_content = "".to_string();
                        self.search_results.clear();
                    }
                }

                Task::none()
            }
            Message::Download => {
                Task::perform(download::download_imgs(true, self.tags.clone()), |()| {
                    Message::ChangeState(State::Viewer)
                })
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.state {
            State::Viewer => {
                let image_handle = self.img_paths.get(self.current_img).unwrap();

                iced::widget::column![
                    iced::widget::row![
                        iced::widget::button("previous")
                            .width(50)
                            .height(30)
                            .on_press(Message::Last),
                        iced::widget::image(image_handle).width(800).height(600),
                        iced::widget::button("next")
                            .width(50)
                            .height(30)
                            .on_press(Message::Next),
                    ],
                    iced::widget::button("del")
                        .width(30)
                        .height(30)
                        .on_press(Message::DeleteCurrent),
                    iced::widget::button("Download")
                        .on_press(Message::ChangeState(State::Download))
                ]
                .into()
            }
            State::Download => {
                let search_bar: iced::widget::TextInput<'_, Message, iced::Theme, iced::Renderer> =
                    iced::widget::TextInput::new("Search", &self.search_bar_content)
                        .width(300)
                        .on_paste(Message::SearchInput)
                        .on_input(Message::SearchInput)
                        .on_submit(Message::AddTag);

                let texts = iced::widget::Column::from_vec(self.format_searchres());

                let search_results = iced::widget::scrollable(texts).height(140);

                let tags: String = self
                    .tags
                    .iter()
                    .map(|s| {
                        let mut sl = s.clone();
                        sl.push(',');
                        sl
                    })
                    .collect();

                let tag_box = iced::widget::text(tags);

                let download_btn = iced::widget::button("Download").on_press(Message::Download);

                let search =
                    iced::widget::column![search_bar, search_results, tag_box, download_btn];
                search.into()
            }
        }
    }

    fn delete_img(&mut self, index: usize) {
        let imgpath = self.img_paths[index].clone();
        if index == self.img_paths.len() - 1 {
            self.current_img -= 1;
        }
        self.img_paths.remove(index);
        let paths: String = self
            .img_paths
            .iter()
            .map(|s| s.to_string() + "\n")
            .collect();
        fs::write("./paths", paths).unwrap();
        if let Ok(()) = fs::remove_file(imgpath) {}
    }

    fn format_searchres<'a>(&self) -> Vec<Element<'a, Message, Theme, iced::Renderer>> {
        let mut searches = Vec::new();

        for sr in self.search_results.clone() {
            match sr.tag_type {
                TagType::Copyright(tag) => searches.push(
                    iced::widget::text(tag)
                        .color(iced::Color::from_rgb(255.0, 0.0, 255.0))
                        .into(),
                ),
                TagType::Character(tag) => searches.push(
                    iced::widget::text(tag)
                        .color(iced::Color::from_rgb(0.0, 170.0, 0.0))
                        .into(),
                ),
                TagType::General(tag) => searches.push(iced::widget::text(tag).into()),
                TagType::Artist(tag) => searches.push(
                    iced::widget::text(tag)
                        .color(iced::Color::from_rgb(170.0, 0.0, 0.0))
                        .into(),
                ),
            }
        }

        searches
    }

    fn update_paths(&mut self) {
        let paths = get_paths();
        let img_paths = get_paths();
        self.img_paths = img_paths;
    }
}

fn get_paths() -> Vec<String> {
    let mut paths = Vec::new();

    let ps = fs::read_to_string("./paths").unwrap();

    for p in ps.lines() {
        paths.push(p.to_string());
    }

    paths
}

fn keys(_v: &Viewer) -> iced::Subscription<Message> {
    use iced::keyboard::{self, key::Named, Key};

    keyboard::on_key_press(|key, _modi| match key.as_ref() {
        Key::Named(Named::ArrowRight) => Some(Message::Next),
        Key::Named(Named::ArrowLeft) => Some(Message::Last),
        Key::Named(Named::Delete) => Some(Message::DeleteCurrent),
        Key::Named(Named::Escape) => Some(Message::Quit),
        _ => None,
    })
}
