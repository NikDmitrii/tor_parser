use clap::Parser;
#[derive(Parser)]
#[command(name = "torrc-updater")]
#[command(about = "Обновляет torrc с мостами", version = "1.0")]
pub struct Args {
    #[arg(short, long, default_value = "/etc/tor/torrc")]
    pub torrc: String,

    #[arg(short, long)]
    pub print: bool,

    #[arg(short, long)]
    pub dry_run: bool,

    #[arg(short = 'r', long = "restart")]
    pub restart: bool,
}
