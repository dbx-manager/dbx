// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn run_app() {
    dbx_lib::run()
    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
//this setup is used so i ("ag") can run the app on to the host directly :>
fn main() {
    if std::env::var("WRAP_EXEC").is_ok() {
        //if running in a distrobox dev container, this code will make sure the app run at the host not the container 
        let args: Vec<String> = std::env::args().skip(1).collect();
        let status = std::process::Command::new("/usr/bin/distrobox-host-exec") 
            .arg(std::env::current_exe().unwrap())
            .args(args)
            .status()
            .expect("Failed to start wrapper");
        
        std::process::exit(status.code().unwrap_or(1));
    } else {
        // Normal execution
        run_app();
    }
}