// #[macro_use]
// extern crate clap;
// use clap::App;

extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use log::info;

mod core;
mod parsing;
mod http;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 解析参数
    // let yaml = load_yaml!("../help.yml");
    // let matches = App::from_yaml(yaml).get_matches();

    // 终端显示logo
    print_lanuch_mascot();

    // Http 服务器
    http::router().await
}

fn print_lanuch_mascot() {
    let ascii_name = r#"
    ___  ___      ________      ________      ________    ________       _______       ________      ________      ________      ___  ___     
    |\  \|\  \    |\   __  \    |\   __  \    |\   __  \  |\   ____\     |\  ___ \     |\   __  \    |\   __  \    |\   ____\    |\  \|\  \    
    \ \  \\\  \   \ \  \|\  \   \ \  \|\  \   \ \  \|\  \ \ \  \___|_    \ \   __/|    \ \  \|\  \   \ \  \|\  \   \ \  \___|    \ \  \\\  \   
     \ \   __  \   \ \  \\\  \   \ \  \\\  \   \ \   ____\ \ \_____  \    \ \  \_|/__   \ \   __  \   \ \   _  _\   \ \  \        \ \   __  \  
      \ \  \ \  \   \ \  \\\  \   \ \  \\\  \   \ \  \___|  \|____|\  \    \ \  \_|\ \   \ \  \ \  \   \ \  \\  \|   \ \  \____    \ \  \ \  \ 
       \ \__\ \__\   \ \_______\   \ \_______\   \ \__\       ____\_\  \    \ \_______\   \ \__\ \__\   \ \__\\ _\    \ \_______\   \ \__\ \__\
        \|__|\|__|    \|_______|    \|_______|    \|__|      |\_________\    \|_______|    \|__|\|__|    \|__|\|__|    \|_______|    \|__|\|__|
                                                             \|_________|                                                                      
    "#;
    
    info!("{}", ascii_name);    
}