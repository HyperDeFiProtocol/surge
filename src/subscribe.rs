use std::fs;


fn req_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let txt = reqwest::blocking::get(url)?.text()?;
    
    Ok(txt)
}

fn read_subscribe_urls() -> Vec<String> {
    println!("Read `./subscribe.list`...");

    let mut urls: Vec<String> = Vec::<String>::new();

    match fs::read_to_string("./subscribe.list") {
        Err(_err) => {
            println!("Failed to read source `./subscribe.list`");
        },
        Ok(text) => {
            for line in text.lines() {
                if line.len() > 0 && &line[..1] != "#" {
                    urls.push(String::from(line.trim()));
                }
            }
        },
    }

    urls
}

pub fn get_nodes() -> Vec<String> {
    let mut lines: Vec<String> = Vec::<String>::new();

    for url in read_subscribe_urls() {
        

        print!("GET URL: `{}`... ", &url);

        match req_get(&url) {
            Err(_err) => {
                println!("Failed!");
            },
            Ok(text) => {
                println!("Succeess!");

                for line in text.lines() {
                    if line.len() > 0 {
                        lines.push(String::from(line));
                    }
                }
                
            },
        }
    }

    lines
}
