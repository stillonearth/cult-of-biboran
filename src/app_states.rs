#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AppState {
    MainMenu,
    FallingGame,
    GameOver,
    GameEnd,
    CutScene,
}
