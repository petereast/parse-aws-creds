use serde::Deserialize;
use serde_json::from_reader;
use std::io::{Result, Stdin};
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
    defaultHeader: bool,

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

fn main() -> Result<()> {
    let options = Options::from_args();

    if options.write {
        panic!("Go and implement this you dick.");
    }
    // Read stdin
    let creds: AwsCredsWrapper = serde_json::from_reader(std::io::stdin())?;

    let output = generate_structure(&creds.Credentials);
    let header = "[default]";

    if options.defaultHeader {
        println!("[default]");
    }
    println!("{}", output.unwrap());

    Ok(())
}
