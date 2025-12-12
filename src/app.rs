use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    MainMenu,
    AnalyzeMenu,
    ExperimentMenu,
    ShowcaseMenu,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuItem {
    Analyze,
    Experiment,
    Showcase,
}

impl MainMenuItem {
    pub fn all() -> [Self; 3] {
        [Self::Analyze, Self::Experiment, Self::Showcase]
    }

    pub fn display(&self) -> &str {
        match self {
            Self::Analyze => "Analyze",
            Self::Experiment => "Experiment",
            Self::Showcase => "Showcase",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnalyzeMenuItem {
    Dataset,
    SampleBatch,
    TrainingGraphs,
    TrainingResults,
}

impl AnalyzeMenuItem {
    pub fn all() -> [Self; 4] {
        [
            Self::Dataset,
            Self::SampleBatch,
            Self::TrainingGraphs,
            Self::TrainingResults,
        ]
    }

    pub fn display(&self) -> &str {
        match self {
            Self::Dataset => "Describe Dataset",
            Self::SampleBatch => "Show Sample Batch",
            Self::TrainingGraphs => "Show Training Graphs",
            Self::TrainingResults => "Show Training Results",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExperimentMenuItem {
    All,
    Selected,
}

impl ExperimentMenuItem {
    pub fn all() -> [Self; 2] {
        [Self::All, Self::Selected]
    }

    pub fn display(&self) -> &str {
        match self {
            Self::All => "Run All Experiments",
            Self::Selected => "Run Selected Experiments",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShowcaseMenuItem {
    Developer,
    Demonstration,
}

impl ShowcaseMenuItem {
    pub fn all() -> [Self; 2] {
        [Self::Developer, Self::Demonstration]
    }

    pub fn display(&self) -> &str {
        match self {
            Self::Developer => "Launch Dashboard - Developer Mode",
            Self::Demonstration => "Launch Dashboard - Demonstration Mode",
        }
    }
}

pub enum Action {
    Continue,
    Quit,
}

pub struct App {
    pub screen: Screen,
    pub selected_main: MainMenuItem,
    pub selected_analyze: AnalyzeMenuItem,
    pub selected_experiment: ExperimentMenuItem,
    pub selected_showcase: ShowcaseMenuItem,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::MainMenu,
            selected_main: MainMenuItem::Analyze,
            selected_analyze: AnalyzeMenuItem::Dataset,
            selected_experiment: ExperimentMenuItem::All,
            selected_showcase: ShowcaseMenuItem::Developer,
        }
    }

    pub fn next(&mut self) -> () {
        match self.screen {
            Screen::MainMenu => {
                self.selected_main = match self.selected_main {
                    MainMenuItem::Analyze => MainMenuItem::Experiment,
                    MainMenuItem::Experiment => MainMenuItem::Showcase,
                    MainMenuItem::Showcase => MainMenuItem::Analyze,
                }
            }
            Screen::AnalyzeMenu => {
                self.selected_analyze = match self.selected_analyze {
                    AnalyzeMenuItem::Dataset => AnalyzeMenuItem::SampleBatch,
                    AnalyzeMenuItem::SampleBatch => AnalyzeMenuItem::TrainingGraphs,
                    AnalyzeMenuItem::TrainingGraphs => AnalyzeMenuItem::TrainingResults,
                    AnalyzeMenuItem::TrainingResults => AnalyzeMenuItem::Dataset,
                }
            }
            Screen::ExperimentMenu => {
                self.selected_experiment = match self.selected_experiment {
                    ExperimentMenuItem::All => ExperimentMenuItem::Selected,
                    ExperimentMenuItem::Selected => ExperimentMenuItem::All,
                }
            }
            Screen::ShowcaseMenu => {
                self.selected_showcase = match self.selected_showcase {
                    ShowcaseMenuItem::Developer => ShowcaseMenuItem::Demonstration,
                    ShowcaseMenuItem::Demonstration => ShowcaseMenuItem::Developer,
                }
            }
        }
    }

    pub fn previous(&mut self) -> () {
        match self.screen {
            Screen::MainMenu => {
                self.selected_main = match self.selected_main {
                    MainMenuItem::Analyze => MainMenuItem::Showcase,
                    MainMenuItem::Experiment => MainMenuItem::Analyze,
                    MainMenuItem::Showcase => MainMenuItem::Experiment,
                }
            }
            Screen::AnalyzeMenu => {
                self.selected_analyze = match self.selected_analyze {
                    AnalyzeMenuItem::Dataset => AnalyzeMenuItem::TrainingResults,
                    AnalyzeMenuItem::SampleBatch => AnalyzeMenuItem::Dataset,
                    AnalyzeMenuItem::TrainingGraphs => AnalyzeMenuItem::SampleBatch,
                    AnalyzeMenuItem::TrainingResults => AnalyzeMenuItem::TrainingGraphs,
                }
            }
            Screen::ExperimentMenu => {
                self.selected_experiment = match self.selected_experiment {
                    ExperimentMenuItem::All => ExperimentMenuItem::Selected,
                    ExperimentMenuItem::Selected => ExperimentMenuItem::All,
                }
            }
            Screen::ShowcaseMenu => {
                self.selected_showcase = match self.selected_showcase {
                    ShowcaseMenuItem::Developer => ShowcaseMenuItem::Demonstration,
                    ShowcaseMenuItem::Demonstration => ShowcaseMenuItem::Developer,
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char('q') => match self.screen {
                Screen::MainMenu => Action::Quit,
                _ => {
                    self.screen = Screen::MainMenu;
                    Action::Continue
                }
            },
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous();
                Action::Continue
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next();
                Action::Continue
            }
            KeyCode::Enter => {
                match self.screen {
                    Screen::MainMenu => match self.selected_main {
                        MainMenuItem::Analyze => {
                            self.screen = Screen::AnalyzeMenu;
                        }
                        MainMenuItem::Experiment => {
                            self.screen = Screen::ExperimentMenu;
                        }
                        MainMenuItem::Showcase => {
                            self.screen = Screen::ShowcaseMenu;
                        }
                    },
                    _ => {}
                }

                Action::Continue
            }
            _ => Action::Continue,
        }
    }
}
