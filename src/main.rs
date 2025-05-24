use std::process::ExitCode;

use std::io;

use std::fs::File;

use rs_zip2asn2data2concat::zipfile2items2stdout;

fn env_val_by_key(key: &'static str) -> impl FnMut() -> Result<String, io::Error> {
    move || {
        std::env::var(key)
            .map_err(|e| format!("missing env var {key}: {e}"))
            .map_err(io::Error::other)
    }
}

fn env2input_zfile_name() -> Result<String, io::Error> {
    env_val_by_key("ENV_INPUT_ZIP_FILENAME")()
}

fn sub() -> Result<(), io::Error> {
    let izname: String = env2input_zfile_name()?;
    let izfile: File = File::open(izname)?;
    zipfile2items2stdout(izfile)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
