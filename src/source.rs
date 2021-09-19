use std::fs;

fn read(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut path = String::from("./source/");
    path.push_str(filename);

    let text = fs::read_to_string(path)?;

    Ok(text)
}

pub fn load(filename: &str) -> String {
    print!("Read `./source/{}`... ", filename);

    let text: String;

    match read(filename) {
        Err(_err) => {
            println!("Failed!\n\n");
            panic!("No preset setting for `{}`...", filename);
        },
        Ok(content) => {
            println!("Success!");
            text = content;
        }
    }

    text
}
