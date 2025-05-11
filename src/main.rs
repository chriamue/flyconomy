use flyconomy::Replay;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "flyconomy", about = "Airline management simulation game.")]
pub struct Opt {
    /// Starts the simulation by loading a replay from a file
    #[structopt(short = "r", long = "replay", parse(from_os_str))]
    replay: Option<std::path::PathBuf>,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let opt = Opt::from_args();

    if let Some(replay_path) = opt.replay {
        let replay = Replay::load_from_file(replay_path).expect("Failed to load replay from file");
        flyconomy::start_from_replay(replay);
    } else {
        flyconomy::start();
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    flyconomy::start();
}
