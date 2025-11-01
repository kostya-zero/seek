use clap::Parser;

/// No-nonsense grep-like tool.
#[derive(Parser)]
#[command(
    name = "ken",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = false,
    arg_required_else_help = false,
)]
pub struct Cli {
    /// The pattern used in search.
    pub pattern: Option<String>,

    /// Snow line number.
    #[arg(short = 'l', long)]
    pub show_line_numbers: bool,

    /// Set the highlight color (red, green, blue, yellow, dimmed, none). Default is red.
    #[arg(short = 'c', long, default_value = "red")]
    pub hightlight_color: String,

    /// Use case-sensitive search.
    #[arg(short, long)]
    pub precise: bool,

    /// Show performance metrics.
    #[arg(short, long)]
    pub metrics: bool,

    /// Print the output as JSON.
    #[arg(short, long)]
    pub json: bool,
}
