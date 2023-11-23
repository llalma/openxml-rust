mod extract;


use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(r"Tests/file-sample_500kB.docx");
    
    extract::extract(&path);
}
