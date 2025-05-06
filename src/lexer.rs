// import necessary modules
use std::fs::File;
use std::io::Read;


fn scanCode(file_lines: String) -> Vec<String> {
    let mut blocks = vec![];
    let mut current_block = String::new();
    let mut brace_depth = 0;

    for line in file_lines.split("\n") {
        
        // Trimming the code
        let trimmed = line.trim();

        // Always add the line to current block
        current_block.push_str(line);

        // Count opening and closing braces
        brace_depth += line.matches('{').count();
        brace_depth -= line.matches('}').count();

        // Check if block ends
        if brace_depth == 0 && (trimmed.ends_with(";") || current_block.contains('{')) {
            blocks.push(current_block.trim().to_string());
            current_block.clear();
        }
    }

    return blocks;
}

pub fn openFile(fileName: &str) -> Vec<String> {
    // Try to open the file
    let mut file: File = File::open(fileName).expect("Failed to open file");

    // Make a string to hold the contents
    let mut contents: String = String::new();

    // Read the file into the string
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Return the string containing the code from the Jade File
    return scanCode(contents);
}