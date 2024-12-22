
use anyhow::Result;
use std::env;
use std::sync::Arc;
use tracing::{error, info, debug};
use super::rig_agent::RigAgent;
use dotenv::dotenv;
use std::io;
use std::io::prelude::*;


pub(crate) async fn generate (req: String) -> Result<String>{

    let rig_agent = Arc::new(RigAgent::new().await?);

    //  // Prompt the user for a message
    //  print!("Enter your message: ");
    //  io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
 
    //  // Read the input from the user
    //  let mut input = String::new();
    //  io::stdin()
    //      .read_line(&mut input)
    //      .expect("Failed to read input");
 
     // Trim the input to remove trailing newline or whitespace
     let message = req.trim().to_string();

     let response = rig_agent.process_message(&message).await?;

     println!("Response: {}", response);
     Ok(response)

} 