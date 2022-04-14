use clap::Parser;
use image::imageops::FilterType;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    pub directory: String,

    #[clap(short, long)]
    pub file_ending: String,

    #[clap(short, long)]
    pub width: u32,

    #[clap(short, long)]
    pub height: u32
}

fn main() {
    let args = Args::parse();

    for entry in WalkDir::new(args.directory) {
        let path = entry.unwrap().path().to_str().unwrap().to_string();

        if path.ends_with(&args.file_ending) {
            println!("Resizing image: {}", &path);

            let mut image = image::open(&path).unwrap();
            let new_image = image.resize(args.width, args.height, FilterType::Nearest);

            new_image.save(&path);
        }
    }
}
