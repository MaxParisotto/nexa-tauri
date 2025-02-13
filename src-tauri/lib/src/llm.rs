use tauri::command;

#[command]
pub async fn chat(message: String) -> Result<String, String> {
    // TODO: Replace with actual LLM integration
    // Example using OpenAI API or local LLM
    Ok(format!("LLM response to: {}", message))
}
