pub enum CurrentScreen {
    Main,
    Projects(ProjectScreen),
    Notes(NoteScreen),
    Settings(SettingScreen),
    Exiting,
}

pub enum ProjectScreen {
    Main,
    Create,
    Open,
}

pub enum NoteScreen {
    Main,
    Create,
    Open,
    Edit
}

pub enum SettingScreen {
    Main,
    Set,
}
