use std::fs;
use std::path::Path;
use std::env;
use std::process;



pub mod helpers {
    use super::*;
    pub fn get_env(){
        let _ = env::set_current_dir("./");
        let path = Path::new(".env");
        let lines = fs::read(path).unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });
        let set_env = |e: String| {
            if let [key, value] = e.split("=").map(|v| v.to_string()).collect::<Vec<String>>().as_slice(){
                env::set_var(key.trim(), value.trim());
            }else {
                return;
            }
        };
        let env_str = String::from_utf8_lossy(&lines.as_slice());
        for e in env_str.split("\n").into_iter() {
            set_env(e.to_string());
        }
    }
}