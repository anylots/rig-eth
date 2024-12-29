#[tokio::test]

async fn gen_agent_tool() {
    use rig::{
        agent::AgentBuilder,
        completion::Prompt,
        loaders::FileLoader,
        providers::openai::{self},
    };
    use std::env;
    use std::{fs, path::Path};

    let cargo_home = env::var("CARGO_HOME").unwrap(); // default ~/.cargo
    let file_path = format!(
        "{}/registry/src/index.crates.io-6f17d22bba15001f/rig-core-0.6.0/examples/agent_with_tools.rs",
        cargo_home
    );

    // Check if a file exists
    if Path::new(&file_path).exists() {
        match fs::read_to_string(file_path.clone()) {
            Ok(content) => {
                println!("The file contents are as follows:\n{}", content);
            }
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                return;
            }
        }
    } else {
        eprintln!("The file path does not exist: {}", file_path);
        return;
    }

    // Create OpenAI client and model
    let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

    //Qwen/Qwen2.5-32B-Instruct
    //Qwen/Qwen2.5-72B-Instruct-128K
    let model = openai_client.completion_model("Qwen/Qwen2.5-32B-Instruct");

    // Load in all the rust examples
    // let data = FileLoader::with_dir("cargo/registry/src/index.crates.io-6f17d22bba15001f/rig-core-0.6.0/examples").unwrap();
    // .cargo/registry/src/index.crates.io-6f17d22bba15001f/rig-core-0.6.0/src/loaders/file.rs
    // .cargo/registry/src/index.crates.io-6f17d22bba15001f/rig-core-0.6.0/examples/agent_with_context.rs
    let examples = FileLoader::with_glob(&file_path)
        .unwrap()
        .read_with_path()
        .ignore_errors()
        .into_iter();

    // Create an agent with multiple context documents
    let agent = examples
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("Rust Example {:?}:\n{}", path, content).as_str())
        })
        .build();

    // Prompt the agent and print the response
    // According to the given examples,
    let response = agent
        .prompt("Please refer to the example to implement an erc20 transfer agent tool with alloy dependency library. The input parameters need to be tokenAddress, toAddress, amount, and privateKey read from env.")
        .await
        .unwrap();

    println!("response: {}", response);
}
