// use std::io::Write;

// use kalosm::{*, language::*};

use kalosm::language::{anyhow, prompt_input, Chat, Llama, LlamaSource, ModelExt, TextStream};

// impl Llama {
//     pub async fn new() -> anyhow::Result<Self> {
//         Llama::builder()
//             .with_source(LlamaSource::llama_7b())
//             .build()
//             .await
//     }
// }

// #[tokio::main]
// async fn main() {
//     // let llm = Llama::new().await.unwrap();
//     let llm = create_llama().await.unwrap();
//
//     // let prompt = "The capital of France is";
//     let prompt = "The following is a 100 word essay about Paris:";
//     print!("{}", prompt);
//
//     let mut stream = llm.stream_text(prompt).with_max_length(1000).await.unwrap();
//
//     stream.to_std_out().await.unwrap();
// }

#[tokio::main]
async fn main() {
    let model = new_chat().await.unwrap();
    let mut chat = Chat::builder(model)
        .with_system_prompt("The assistant will act like a pirate")
        .build();

    loop {
        chat.add_message(prompt_input("\n> ").unwrap())
            .to_std_out()
            .await
            .unwrap();
    }
}

pub async fn new_chat() -> anyhow::Result<Llama> {
    Llama::builder()
        .with_source(LlamaSource::llama_3_2_3b_chat())
        .build()
        .await
}

async fn create_llama()-> anyhow::Result<Llama>{
    Llama::builder()
        .with_source(LlamaSource::llama_3_2_3b_chat())
        .build()
        .await
    // Llama::builder()
    //     .with_source(LlamaSource::llama_8b())
    //     .build()
    //     .await
}