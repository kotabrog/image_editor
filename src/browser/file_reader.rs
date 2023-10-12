use anyhow::{anyhow, Result};
use web_sys::{
    FileReader, File,
};

pub fn file_reader() -> Result<FileReader> {
    FileReader::new()
        .map_err(|err| anyhow!("Could not create FileReader {:#?}", err))
}

pub fn file_reader_result(file_reader: &FileReader) -> Result<String> {
    file_reader.result()
        .map_err(|err| anyhow!("Could not get result from FileReader {:#?}", err))?
        .as_string()
        .ok_or_else(|| anyhow!("Could not get result as string from FileReader"))
}

pub fn file_reader_read_as_data_url(file_reader: &FileReader, file: &File) -> Result<()> {
    file_reader.read_as_data_url(file)
        .map_err(|err| anyhow!("Could not read file {:#?}", err))
}
