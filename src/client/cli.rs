use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "syt")]
#[command(about = "Safe your Time", long_about = None)]
struct Syt {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new entry
    Create(CreateArgs),
    /// Update an existing entry
    Update(UpdateArgs),
    /// Delete an entry
    Delete(DeleteArgs),
    /// Show status
    Status(StatusArgs),
    /// Start an entry
    Start(StartArgs),
    /// Stop an entry
    Stop(StopArgs),
    /// Show an entry
    Show(ShowArgs),
    /// List all entrys
    List(ListArgs),
    /// Configuration settings
    Config,
}

#[derive(Args, Debug)]
struct CreateArgs {
    /// Entry name (can also be passed without flag)
    #[arg(short, long, required = true)]
    name: String,
    /// Duration (cannot be used with start-time)
    #[arg(short, long)]
    duration: Option<String>,
    /// Start time (requires end-time)
    #[arg(short = 's', long)]
    start_time: Option<String>,
    /// End time (requires start-time)
    #[arg(short = 'e', long)]
    end_time: Option<String>,
    /// Message
    #[arg(short, long)]
    message: Option<String>,
    /// Blocked websites
    #[arg(short = 'w', long)]
    denied_web: Option<Vec<String>>,
    /// Allowed websites
    #[arg(short = 'W', long)]
    allow_web: Option<Vec<String>>,
    /// Blocked applications
    #[arg(short = 'a', long)]
    denied_apps: Option<Vec<String>>,
    /// Allowed applications
    #[arg(short = 'A', long)]
    allow_apps: Option<Vec<String>>,
    /// Days
    #[arg(short = 'y', long)]
    days: Option<String>,
}

#[derive(Args, Debug)]
struct UpdateArgs {
    /// Name of entry to update
    name: String,
    /// New name
    #[arg(short, long)]
    new_name: Option<String>,
    /// Duration (cannot be used with start-time)
    #[arg(short, long)]
    duration: Option<String>,
    /// Start time (requires end-time)
    #[arg(short = 's', long)]
    start_time: Option<String>,
    /// End time (requires start-time)
    #[arg(short = 'e', long)]
    end_time: Option<String>,
    /// Message
    #[arg(short, long)]
    message: Option<String>,
    /// Blocked websites
    #[arg(short = 'w', long)]
    denied_web: Option<Vec<String>>,
    /// Allowed websites
    #[arg(short = 'W', long)]
    allow_web: Option<Vec<String>>,
    /// Blocked applications
    #[arg(short = 'a', long)]
    denied_apps: Option<Vec<String>>,
    /// Allowed applications
    #[arg(short = 'A', long)]
    allow_apps: Option<Vec<String>>,
    /// Days
    #[arg(short = 'y', long)]
    days: Option<String>,
}

#[derive(Args, Debug)]
struct DeleteArgs {
    /// Entry name (can also be passed with --name)
    #[arg(short, long)]
    name: Option<String>,
    #[arg(required = true)]
    name_positional: Option<String>,
}

#[derive(Args, Debug)]
struct StatusArgs {
    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Args, Debug)]
struct StartArgs {
    /// Entry name (can also be passed with --name)
    #[arg(short, long)]
    name: Option<String>,
    #[arg(required = true)]
    name_positional: Option<String>,
}

#[derive(Args, Debug)]
struct StopArgs {
    /// Entry name (can also be passed with --name)
    #[arg(short, long)]
    name: Option<String>,
    #[arg(required = true)]
    name_positional: Option<String>,
}

#[derive(Args, Debug)]
struct ShowArgs {
    /// Entry name (can also be passed with --name)
    #[arg(short, long)]
    name: Option<String>,
    #[arg(required = true)]
    name_positional: Option<String>,
}

#[derive(Args, Debug)]
struct ListArgs {}

fn main() {
    let syt = Syt::parse();
    println!("{syt:?}");
}
