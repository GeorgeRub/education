use std::io::Write;

use kalosm::{*, language::*};

#[tokio::main]
async fn main() {
    let mut llm = Llama::new().await.unwrap();
    let prompt = "The following is a 300 word essay about Paris:";
    print!("{}", prompt);

    let mut stream = llm.stream_text(prompt).with_max_length(1000).await.unwrap();

    stream.to_std_out().await.unwrap();
}