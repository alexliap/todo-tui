pub enum CurrentScreen {
    Main,
    Projects(ProjectScreen),
    Settings(SettingScreen),
    Exiting,
}

pub enum ProjectScreen {
    Main,
    Create,
    // OpenProject,
}

pub enum SettingScreen {
    Main,
    Set,
    // OpenProject,
}
