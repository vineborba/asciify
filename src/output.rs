use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

use crate::options::Output;

pub fn prepare_output_buffer(
    output_method: &Output,
    file_path: &String,
) -> Result<BufWriter<Box<dyn Write>>> {
    match output_method {
        Output::Stdout => {
            let output_wrapper = Box::new(std::io::stdout().lock());
            Ok(BufWriter::with_capacity(1024, output_wrapper))
        }
        Output::File => {
            let output_file = Box::new(File::create(file_path)?);
            Ok(BufWriter::with_capacity(1024, output_file))
        }
    }
}
