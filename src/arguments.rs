use clap::Parser;

#[derive(Parser)]
#[clap(author = "Bart Kessels", version, about = "Small CLI to setup your computer in no time")]
pub struct Arguments {
    #[arg(short = 'c', long = "configuration-file")]
    pub configuration_file: String
}