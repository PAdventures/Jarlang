use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Run a Jarlang file by giving the entry point,
/// or start a REPL session.
///
/// Note: When in a REPL session, you can exit the session by typing "exit", Ctrl + D or Ctrl + C
pub struct CLI {
    /// The path to your Jarlang entry file
    pub path: Option<String>,
}
