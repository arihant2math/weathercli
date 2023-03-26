use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct WeatherFile {
    pub path: PathBuf,
    #[pyo3(get, set)]
    pub data: String,
    #[pyo3(get)]
    pub exists: bool,
}

#[pymethods]
impl WeatherFile {
    #[new]
    pub fn new(file_name: String) -> Self {
        let mut path = dirs::home_dir().expect("expect home dir");
        let mut exists = true;
        path.push(".weathercli");
        if !path.exists() {
            fs::create_dir_all(path.display().to_string()).expect("dir creation failed");
        }
        path.push(file_name);
        if !path.exists() {
            exists = false;
            let mut file = File::create(path.display().to_string()).expect("file creation failed");
            file.write_all(b"{}")
                .expect("Could not write to newly created file");
        }
        let file = File::open(path.display().to_string()).expect("File Open Failed");
        let mut buf_reader = BufReader::new(file);
        let mut data = String::new();
        buf_reader.read_to_string(&mut data).expect("Read failed");
        WeatherFile { path, data, exists }
    }

    /// Writes self.data to the file at self.path
    pub fn write(&self) {
        let f = File::options()
            .write(true)
            .truncate(true)
            .open(self.path.display().to_string())
            .expect("File opening failed");
        let mut f = BufWriter::new(f);
        f.write_all(self.data.as_bytes())
            .expect("Unable to write data");
        f.flush().expect("Flushing failed");
    }

    #[getter]
    pub fn get_path(&self) -> String {
        self.path.display().to_string()
    }
}
