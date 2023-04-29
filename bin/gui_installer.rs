use iced::{Element, Length, Sandbox, Settings};
use iced::alignment;
use iced::theme;
use iced::widget::{checkbox, column, container, horizontal_space, row, scrollable, text};
use iced::widget::{Button, Column};

pub struct Installer {
    steps: Steps,
}

impl Sandbox for Installer {
    type Message = Message;

    fn new() -> Installer {
        Installer {
            steps: Steps::new(),
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Installer { steps, .. } = self;

        let mut controls = row![];

        if steps.can_revert() {
            controls = controls.push(
                button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button("Next")
                    .on_press(Message::NextPressed)
                    .style(theme::Button::Primary),
            );
        }

        let content: Element<_> = column![steps.view().map(Message::StepMessage), controls,]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

        let scrollable = scrollable(container(content).width(Length::Fill).center_x());

        container(scrollable).height(Length::Fill).center_y().into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Components {
                    daemon_checked: false,
                    updater_checked: true,
                },
                Step::Install { completed: false },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.can_revert() {
            self.current -= 1;
        }
    }

    fn can_revert(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_revert()
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    Welcome,
    Components {
        daemon_checked: bool,
        updater_checked: bool,
    },
    Install {
        completed: bool,
    },
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    DaemonChecked(bool),
    UpdaterChecked(bool),
    WeatherCliChecked(bool),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::DaemonChecked(v) => {
                if let Step::Components { daemon_checked, .. } = self {
                    *daemon_checked = v;
                }
            }
            StepMessage::UpdaterChecked(v) => {
                if let Step::Components {
                    daemon_checked,
                    updater_checked,
                } = self
                {
                    *updater_checked = v;
                }
            }
            StepMessage::WeatherCliChecked(v) => {}
        };
    }

    fn title(&self) -> &str {
        "Installer - WeatherCLI"
    }

    fn can_revert(&self) -> bool {
        match self {
            Step::Welcome => false,
            Step::Components { .. } => true,
            Step::Install { completed } => *completed,
            Step::End => false,
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Components { .. } => true,
            Step::Install { completed } => *completed,
            Step::End => false,
        }
    }

    fn view(&self) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Components {
                daemon_checked,
                updater_checked,
            } => Self::components(*daemon_checked, *updater_checked),
            Step::Install { completed } => Self::install(*completed),
            Step::End => Self::end(),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        column![text(title).size(50)].spacing(20)
    }

    fn welcome() -> Column<'a, StepMessage> {
        Self::container("WeatherCLI Installer")
            .push("Welcome to the GUI weathercli installer, press Next to begin.")
    }

    fn components(daemon_checked: bool, updater_checked: bool) -> Column<'a, StepMessage> {
        let weather_cli = checkbox("WeatherCLI", true, StepMessage::WeatherCliChecked);
        let daemon = checkbox("Daemon", daemon_checked, StepMessage::DaemonChecked);
        let updater = checkbox("Updater", updater_checked, StepMessage::UpdaterChecked);
        Self::container("Components")
            .push("Choose which components you wish to install, these can be downloaded later if you wish")
            .push(weather_cli)
            .push(daemon)
            .push(updater)
    }

    fn install(completed: bool) -> Column<'a, StepMessage> {
        let install_btn = button("Install");
        Self::container("Install")
            .push("Please wait as weathercli installs ...")
            .push(install_btn)
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("The installation has completed!").push("You may close this window.")
    }
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(text(label).horizontal_alignment(alignment::Horizontal::Center))
        .padding(12)
        .width(100)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

fn main() -> Result<(), String> {
    Installer::run(Settings::default()).expect("GUI Failed");
    Ok(())
}
