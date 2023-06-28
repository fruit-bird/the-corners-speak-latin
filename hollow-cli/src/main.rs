#[cfg(feature = "clipboard")]
use arboard::Clipboard;
use clap::Parser;
use hollow::Hollow;

#[derive(Parser)]
struct HollowCLI {
    /// Wikipedia topic/link to any article
    #[arg(default_value = "Rumpelstiltskin")]
    first_topic: String,
    /// Wikipedia topic/link to another article
    #[arg(default_value = "Moon landing conspiracies")]
    second_topic: String,
    /// Language to mix into the articles
    #[arg(short = 'l', long = "lang", default_value = "ja")]
    second_language: String,
    #[cfg(feature = "clipboard")]
    /// Copy the output to the clipboard
    #[arg(short, long)]
    clipboard: bool,
}

#[tokio::main]
async fn main() {
    let args = HollowCLI::parse();
    let hollow = Hollow::new(&args.first_topic, &args.second_topic, &args.second_language);

    let the_spooky = match hollow.run().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };
    println!("{}", the_spooky);

    #[cfg(feature = "clipboard")]
    if args.clipboard {
        if let Err(e) = Clipboard::new().and_then(|mut c| c.set_text(the_spooky)) {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}
