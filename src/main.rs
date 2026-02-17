mod fetch;
mod render;
mod cache;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tldr-rust", version, about)]

struct Args{
    command: Option<String>,

    #[arg(short, long)]
    update: bool,

    #[arg(short, long, default_value = "common")]
    platform: String,
}

fn main(){
    let args = Args::parse();

    //kalo ga ada command, kasih hint

    let command = match args.command{
        Some(cmd) => cmd,
        None => {
            eprintln!("Usage: tldr-rust <command>\nExample: tldr-rust tar");
            std::process::exit(1);

        }
    };

    if !args.update{
        if let Some(cached) = cache::load(&command){
            render::print_page(&cached);
            return;

        }
    }

    //ambil dari CDN
    println!("Fetching page for '{}'...", command);
    match fetch::get_page(&command, &args.platform){
        Ok(content) => {
            cache::save(&command, &content);
            render::print_page(&content);

        }
        Err(e) => {
            eprintln!("Error: could not find page for '{}'\n{}", command, e);
            std::process::exit(1);
        }
    }
}