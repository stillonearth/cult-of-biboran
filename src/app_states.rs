#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AppState {
    MainMenu,
    InGame,
    GameOver,
    CutScene,
}
