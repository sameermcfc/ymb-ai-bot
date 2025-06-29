use anyhow::{Context, Result};
use rig::providers::gemini::{self, completion::gemini_api_types::GenerationConfig};
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::agent::Agent;
use rig::completion::Prompt;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use std::time::Instant;



pub struct CalendarAgent {
    agent: Arc <Agent<gemini::completion::CompletionModel>>,
}


impl CalendarAgent {

    fn load_md_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
        fs::read_to_string(file_path.as_ref())
            .with_context(|| format!("Failed to read markdown file: {:?}", file_path.as_ref()))
    }

    pub async fn process_message(&self, message: &str) -> Result<String> {
        self.agent.prompt(message).await.map_err(anyhow::Error::from)
    }

    pub async fn new() -> Result<Self> {
        //Initialse gemini client
        let gemini_client = gemini::Client::from_env();
        let embedding_model = gemini_client.embedding_model(gemini::embedding::EMBEDDING_004);

        //create vector store
        let mut vector_store = InMemoryVectorStore::default();

        //Get current working directory and construct path to md files for embedding
        let current_dir = std::env::current_dir()?;
        let documents_dir = current_dir.join("documents");
        let md1_path = documents_dir.join("calendar_concepts.md");
        let md2_path = documents_dir.join("event_summary_guidelines.md");
        let md3_path = documents_dir.join("locations.md");
        let t1 = Instant::now();
        let md1_content = Self::load_md_content(&md1_path);
        let md2_content = Self::load_md_content(&md2_path);
        let md3_content = Self::load_md_content(&md3_path);
        println!("Loaded files in {:?}", t1.elapsed());

        

        //Create embeddings and add to vector store
        let t1 = Instant::now();
        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .document(md1_content.unwrap())?
        .build()
        .await?;    

        vector_store.add_documents(embeddings);
        println!("added documents to vectors in {:?}", t1.elapsed());
        //create index
        let t1 = Instant::now();
        let index = vector_store.index(embedding_model);

        let t1 = Instant::now();
        let agent = Arc::new(gemini_client.agent(gemini::completion::GEMINI_1_5_FLASH)
        .preamble(include_str!("../../prompts/calendar_preamble.txt"))
        .dynamic_context(2, index)
        .additional_params(serde_json::to_value(GenerationConfig {
            response_mime_type:  Some("application/json".to_string()),
            ..Default::default()
        })?) // Unwrap the Result to get the Value
        .build());

        Ok(Self { agent })
    }


}


