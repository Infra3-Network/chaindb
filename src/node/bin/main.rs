use chaindb_node::cli;
use chaindb_common::Error;
// #[derive(Debug, Parser)]
// pub enum Command {
//     /// Generate a random node key for p2p peer.
//     GenerateNodeKey(GenerateNodeKeyCmd),
// }

// fn run() -> Result<(), Error> {
//     match Command::parse() {
//         Command::GenerateNodeKey(cmd) => cmd.run(),
//     }
// }

fn main() -> Result<(), Error> {
    cli::run();
    Ok(())
}
