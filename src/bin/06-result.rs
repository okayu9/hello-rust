use std::fs;

fn main(){
    let path = "Cargo.toml";
    let result = fs::read_to_string(path);

    match result {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Failed to read file: {}", e),
    }

    match read_file_contents("hidden_tome_of_secrets.txt") // おそらく存在しないファイル
    {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Failed to read file: {}", e),
    }
}

fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
