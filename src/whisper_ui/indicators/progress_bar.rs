use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::File,
    io::{self, Read},
};

use crate::whisper_core::exceptions::whisper_error::WhisperError;

pub fn create_progress_reader(
    file: File,
    progress_bar: ProgressBar,
) -> Result<ProgressReader<File>, WhisperError> {
    let reader = ProgressReader::new(file, progress_bar);
    Ok(reader)
}

pub fn create_progress_bar(file_size: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .expect("Failed to create progress bar template")
            .progress_chars("#>-"),
    );
    progress_bar
}

pub struct ProgressReader<R: Read> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: Read> ProgressReader<R> {
    fn new(inner: R, progress_bar: ProgressBar) -> Self {
        Self {
            inner,
            progress_bar,
        }
    }
}

impl<R: Read> Read for ProgressReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.progress_bar.inc(bytes_read as u64);
        Ok(bytes_read)
    }
}
