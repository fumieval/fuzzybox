use clap::Parser;
use std::io::Read;

mod options;
mod union_find;
mod uniq;

#[tokio::main]
async fn main() {
    let opts = options::Cli::parse();
    match opts.subcommand {
        options::SubCommands::Ask { question, json } => {
            ask(&opts.openai_api_key, &question.as_deref(), json).await
        }
        options::SubCommands::Uniq { threshold, dot } => {
            uniq::uniq(&opts.openai_api_key, threshold, dot).await
        }
    }
}

async fn ask(api_key: &str, question: &Option<&str>, json: bool) {
    let client = openai_rust::Client::new(api_key);
    let mut input = question.unwrap_or("").to_owned();

    // use stdin if it's being piped in
    if atty::isnt(atty::Stream::Stdin) {
        std::io::stdin().read_to_string(&mut input).unwrap();
    }

    let mut args = openai_rust::chat::ChatArguments::new(
        "gpt-4o",
        vec![openai_rust::chat::Message {
            role: "user".to_owned(),
            content: input,
        }],
    );
    args.response_format = if json {
        Some(openai_rust::chat::ResponseFormat {
            type_: openai_rust::chat::ResponseFormatType::JsonObject,
        })
    } else {
        None
    };
    let res = client.create_chat(args).await.unwrap();
    print!("{}", res);
}
