mod bindings {
    use super::StandupPlugin;

    wit_bindgen::generate!({
        with: {
            "wasi:cli/environment@0.2.0": ::wasi::cli::environment,
            "wasi:io/error@0.2.0": ::wasi::io::error,
            "wasi:io/poll@0.2.0": ::wasi::io::poll,
            "wasi:io/streams@0.2.0": ::wasi::io::streams,
        },
        generate_all
    });

    export!(StandupPlugin);
}

use bindings::exports::wasmcloud::wash::subcommand::{
    Argument, Guest as SubcommandGuest, Metadata,
};

use wasi::exports::cli::run::Guest as RunGuest;

#[derive(Debug, serde::Deserialize)]
struct StandupData {
    name: String,
    roll: u32,
}

struct StandupPlugin;

// Our implementation of the wasi:cli/run interface
impl RunGuest for StandupPlugin {
    fn run() -> Result<(), ()> {
        let env_user = std::env::var("STANDUP_NAME").ok();
        let user = std::env::args().nth(1).or(env_user).unwrap_or_else(|| {
            eprintln!("Make sure to provide a name to roll the standup initiative for as an arg");
            std::process::exit(1);
        });

        // Check first to make sure the user hasn't already rolled
        let mut response = futures::executor::block_on(
            reqwest::Client::new()
                .get("https://standup.cosmonic.sh/api/initiative")
                .header("Content-Type", "application/json")
                .send(),
        )
        .expect("should get response bytes");
        let current_initiatives: Vec<StandupData> =
            serde_json::from_reader(&mut response.bytes_stream().expect("should get bytes stream"))
                .expect("should deserialize response");
        if current_initiatives.iter().any(|i| i.name == user) {
            println!(
                "You already rolled today, good job {}.\nGet on over to https://standup.cosmonic.sh/",
                user
            );
            return Ok(());
        }

        // Roll initiative
        let mut response = futures::executor::block_on(
            reqwest::Client::new()
                .post("https://standup.cosmonic.sh/api/initiative")
                .header("Content-Type", "application/json")
                .body(format!("{{\"name\": \"{user}\", \"created_at\": 0}}"))
                .send(),
        )
        .expect("should get response bytes");
        let standup_response: StandupData =
            serde_json::from_reader(&mut response.bytes_stream().expect("should get bytes stream"))
                .expect("should deserialize response");
        println!(
            "You rolled a {}, good job {}.\nGet on over to https://standup.cosmonic.sh/",
            standup_response.roll, standup_response.name
        );

        Ok(())
    }
}

// Our plugin's metadata implemented for the subcommand interface
impl SubcommandGuest for StandupPlugin {
    fn register() -> Metadata {
        Metadata {
            name: "Standup Plugin".to_string(),
            id: "standup".to_string(),
            description: "Let wash roll your standup initiative".to_string(),
            author: "Brooks".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            flags: vec![],
            arguments: vec![(
                "name".to_string(),
                Argument {
                    required: false,
                    description: "The name of the person to roll the standup initiative for. Required unless STANDUP_NAME environment variable is set."
                        .to_string(),
                    is_path: false,
                },
            )],
        }
    }
}

wasi::cli::command::export!(StandupPlugin);
