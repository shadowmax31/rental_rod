use crate::util::file;
use crate::db::line::Line;
use super::v1::line_to_str::line_to_str;
use super::v1::reader;


const TBL_EXT: &str = ".dt";

pub struct TableManagerV1 {
    tbl_path: String
}

impl TableManagerV1 {
    pub fn new(base_path: &str, tbl: &str) -> TableManagerV1 {
        let with_ext = tbl.to_owned() + TBL_EXT;

        let fullpath = std::path::Path::new(&base_path).join(with_ext); 
        let fullpath = fullpath.to_str().unwrap_or("");
        if fullpath == "" {
            panic!("The path to the table is empty");
        }

        TableManagerV1 {
            tbl_path: String::from(fullpath)
        }
    }

    pub fn insert(&self, line: &Line) -> Result<(), std::io::Error> {
        self.create_table()?;


        file::insert(&self.tbl_path, &line_to_str(line))?;

        Ok(())
    }

    pub fn read(&self) -> Vec<Line> {
        let raw = file::read(&self.tbl_path).unwrap_or(Vec::new());

        reader::read(&raw)
    }
    
    fn create_table(&self) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&self.tbl_path);
        if !path.exists() {
            file::insert(&self.tbl_path, "#v1.0")?;
        }

        Ok(())
    }
}

