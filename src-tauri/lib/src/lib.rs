mod llm;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            llm::chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
