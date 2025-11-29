use std::path::PathBuf;

use std::error::Error;
use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use std::process;

use clap::Parser;
use vst3_bindgen::generate;

#[derive(Parser)]
struct Vst3Bindgen {
    sdk_dir: PathBuf,

    #[clap(long, value_name = "TRIPLE")]
    target: Option<String>,

    #[clap(long, short, value_name = "OUTPUT")]
    output: Option<String>,
}

fn vst3_bindgen(cmd: &Vst3Bindgen) -> Result<(), Box<dyn Error>> {
    let mut bindings = Vec::new();

    generate(&cmd.sdk_dir, cmd.target.as_deref(), &mut bindings)?;

    if let Some(output) = &cmd.output {
        let file = File::create(output)?;
        BufWriter::new(file).write_all(&bindings)?;
    } else {
        BufWriter::new(stdout()).write_all(&bindings)?;
    };

    Ok(())
}

fn main() {
    let cmd = Vst3Bindgen::parse();
    if let Err(err) = vst3_bindgen(&cmd) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
