fn main() {
    let content = "gantt\nMPU EMG Flex : 1d";
    match merman::parse(content) {
        Ok(_) => println!("Parsed OK"),
        Err(e) => println!("Error: {}", e),
    }
}
