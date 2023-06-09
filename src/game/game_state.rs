#[derive(PartialEq, Default, Debug)]
pub enum GameState {
    #[default]
    Welcome,
    Playing,
    GameOver,
}
