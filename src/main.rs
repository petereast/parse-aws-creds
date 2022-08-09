use directories::UserDirs;
use serde::Deserialize;
use std::fs::File;
use std::io::{prelude::*, Result, Stdin, Write};
use structopt::StructOpt;

#[derive(Deserialize, Debug)]
struct AwsCredsWrapper {
    Credentials: AwsCredsResponse,
}

#[derive(Deserialize, Debug)]
struct AwsCredsResponse {
    SecretAccessKey: Option<String>,
    AccessKeyId: Option<String>,
    SessionToken: Option<String>,
}

#[derive(StructOpt, Debug)]
struct Options {
    // Prefix the output with "[default]"?
    #[structopt(short, long)]
    default_header: bool,

    // Write to ~/.aws/credentials directly
    #[structopt(short, long)]
    write: bool,
}

fn generate_structure(input: &AwsCredsResponse) -> Option<String> {
    Some(
        [
            ("aws_secret_access_key", input.SecretAccessKey.as_ref()?),
            ("aws_access_key_id", input.AccessKeyId.as_ref()?),
            ("aws_session_token", input.SessionToken.as_ref()?),
        ]
        .map(|(k, v)| format!("{k} = {v}", k = k, v = v))
        .join("\n"),
    )
}

fn get_writer(is_file: bool) -> Box<dyn Write> {
    if is_file {
        let user_dirs = UserDirs::new().expect("Can't get home dir");
        let home_dir = user_dirs.home_dir();

        Box::new(File::create(home_dir.join(".aws/credentials").clone()).unwrap())
    } else {
        Box::new(std::io::stdout())
    }
}

fn main() -> Result<()> {
    let options = Options::from_args();

    // Read stdin
    let creds: AwsCredsWrapper = serde_json::from_reader(std::io::stdin())?;

    let mut output_stream = get_writer(options.write);
    if options.default_header {
        output_stream.write_all(b"[default]\n")?;
    }

    output_stream.write_all(
        generate_structure(&creds.Credentials)
            .expect("Can't generate creds")
            .as_bytes(),
    )?;

    Ok(())
}
