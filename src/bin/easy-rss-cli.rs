extern crate easy_rss_lib;


use easy_rss_lib::config::*;
use easy_rss_lib::app::*;


fn print_help(){
    println!("{}", r#"
Usage: easy-rss-cli [Option]
Option:
    [filename]: config file [json]

Example:
    easy-rss-cli zhihu.json # Request to get RSS data
"#)
}


fn main()->Result<(),Box<dyn std::error::Error>> {
    // File
    let args:Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        print_help();
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound,"Not Found Config")));
    }

    let filename = args.last().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound,"Not Found Config"))?;
    // Config
    let mut conf = CliConfig::from(filename.as_str())?;

    App::from(&mut conf).run()?;
    Ok(())
}
