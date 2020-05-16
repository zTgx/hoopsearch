// #[macro_use]
// extern crate clap;
// use clap::App;

// #[macro_use] 
extern crate log;
use log::info;

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 解析参数
    // let yaml = load_yaml!("../help.yml");
    // let matches = App::from_yaml(yaml).get_matches();

    // 终端显示logo
    print_lanuch_mascot();
}

fn print_lanuch_mascot() {
    let ascii_name = r#"
    hhh       hhh
    hhh       hhh                                          .d8888b.                                    888
    hhh       hhh                                         d88P  Y88b                                   888  
    hhh       hhh                         ppp             Y88b.                                        888
    hhhhhhhhhhhhh    oooooo       oooooo   ppp    pppppp    "Y888b.    .d88b.   8888b.  888d888 .d8888b 88888b.
    hhh       hhh  oooooooooo   oooooooooo  ppp pppppppppp      "Y88b. d8P  Y8b     "88b 888P"  d88P"    888 "88b
    hhh       hhh oooooooooooo oooooooooooo ppppppppppppppp       "888 88888888 .d888888 888    888      888  888
    hhh       hhh  oooooooooo   oooooooooo  ppp pppppppppp  Y88b  d88P Y8b.     888  888 888    Y88b.    888  888
    hhh       hhh   ooooooo      ooooooo    ppp  ppppppp    "Y8888P"   "Y8888  "Y888888 888     "Y8888P 888  888     
                                            ppp   
                                            ppp  
                                            ppp
                                           ppp
                                          ppp
    "#;
    
    info!("{}", ascii_name);    
}