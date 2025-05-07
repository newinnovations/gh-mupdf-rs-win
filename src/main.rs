use mupdf::Document;
use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/pdf_file.pdf>", args[0]);
        return Err("Invalid arguments".into());
    }
    let pdf_path = Path::new(&args[1]);

    if !pdf_path.exists() {
        return Err(format!("PDF file not found: {}", pdf_path.display()).into());
    }
    if !pdf_path.is_file() {
        return Err(format!("Path is not a file: {}", pdf_path.display()).into());
    }

    println!("Opening PDF: {} ...", pdf_path.display());
    let doc = Document::open(pdf_path.to_str().ok_or("Invalid PDF path encoding")?)?;
    println!("PDF opened successfully.");

    let page_count = doc.page_count()?;
    println!("Total pages: {}", page_count);

    Ok(())
}
