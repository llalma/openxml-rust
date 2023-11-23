use std::fs::File;
use std::path::PathBuf;
use zip::ZipArchive;
use std::fs;
use std::io;

extern crate tempdir;
use tempdir::TempDir;

pub fn extract(path: &PathBuf) -> PathBuf{
    
    // Create a temp directory for unziped file
    let tmp_dir = TempDir::new("").unwrap();  
    
    // Open the file as an archive
    let file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        
        // Determine if a folder
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        
        // Is a file
        } else {
            // Create any parent dirs of file if needed
            if let Some(p) = outpath.parent() {
                let parent_path = tmp_dir.path().join(&p);
                if !parent_path.exists() {
                    fs::create_dir_all(parent_path).unwrap();
                }
            }

            // Write the actual file
            let mut outfile = fs::File::create(tmp_dir.path().join(&outpath)).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    return tmp_dir.into_path()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::path::Path;

    extern crate glob;
    use glob::glob;

    #[test]
    fn test_word() {

        let expected = vec![
            "[Content_Types].xml",
            "_rels",
            "_rels/.rels",
            "docProps",
            "docProps/app.xml",
            "docProps/core.xml",
            "word",
            "word/_rels",
            "word/_rels/document.xml.rels",
            "word/charts",
            "word/charts/chart1.xml",
            "word/document.xml",
            "word/fontTable.xml",
            "word/media",
            "word/media/image1.jpeg",
            "word/numbering.xml",
            "word/settings.xml",
            "word/styles.xml",
        ];

        let path = PathBuf::from(r"src/Tests/file-sample_500kB.docx");
        let tmp_dir: PathBuf = extract(&path);
        
        let search_path = Path::new("")
            .join(tmp_dir.into_os_string().into_string().unwrap())
            .join("**/*");
        
        let files: Vec<PathBuf> = glob(
                &search_path
                .into_os_string()
                .into_string()
                .unwrap())
            .unwrap()
            .map(|v| v.unwrap())
            .collect();

        for (a, e) in files.into_iter().zip(expected){
            assert!(a.into_os_string()
            .into_string()
            .unwrap()
            .ends_with(e));
        }
    }

}
