use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::BufWriter;
use std::fs::File;
use std::ptr;

use calamine::{open_workbook_auto, Reader};
use csv::WriterBuilder;

#[repr(C)]
pub enum ConversionResult {
    Success = 0,
    InvalidInputFilename = 1,
    InvalidOutputFilename = 2,
    ExcelReadError = 3,
    NoSheetsFound = 4,
    WorksheetRangeError = 5,
    CsvFileCreateError = 6,
    CsvWriteError = 7,
    OtherError = 99,
}

fn write_error_message(buffer: *mut c_char, buffer_len: usize, message: &str) {
    if buffer.is_null() || buffer_len == 0 {
        return;
    }

    let c_string = match CString::new(message) {
        Ok(s) => s,
        Err(_) => CString::new("Error: Message contains null bytes.").unwrap(),
    };

    let bytes = c_string.as_bytes_with_nul();

    let len_to_copy = std::cmp::min(bytes.len(), buffer_len);
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr() as *const c_char,
            buffer,
            len_to_copy,
        );

        if len_to_copy > 0 {
            *buffer.add(len_to_copy - 1) = 0;
        } else if buffer_len > 0 {
            *buffer = 0;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn convert_excel_to_csv(
    filename_ptr: *const c_char,
    output_filename_ptr: *const c_char,
    error_message_buf: *mut c_char,
    error_message_buf_len: usize,
) -> ConversionResult {
    if filename_ptr.is_null() || output_filename_ptr.is_null() {
        write_error_message(
            error_message_buf,
            error_message_buf_len,
            "Null pointer for filename provided.",
        );

        return ConversionResult::OtherError;
    }

    let filename = match unsafe { CStr::from_ptr(filename_ptr) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            write_error_message(
                error_message_buf,
                error_message_buf_len,
                "Input filename contains invalid UTF-8 characters.",
            );

            return ConversionResult::InvalidInputFilename;
        }
    };

    let output_filename = match unsafe {
        CStr::from_ptr(output_filename_ptr)
    }.to_str() {
        Ok(s) => s,
        Err(_) => {
            write_error_message(
                error_message_buf,
                error_message_buf_len,
                "Output filename contains invalid UTF-8 characters.",
            );

            return ConversionResult::InvalidOutputFilename;
        }
    };

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let excel_file_path = filename;
        let csv_file_path = output_filename;
        let sheet_name_to_convert: Option<&str> = None;

        let mut workbook = open_workbook_auto(excel_file_path)
            .map_err(|e| format!("Failed to open Excel workbook: {}", e))?;

        let sheet_name = if let Some(name) = sheet_name_to_convert {
            name.to_string()
        } else {
            workbook.sheet_names().first()
                .ok_or_else(|| "No sheets found in workbook".to_string())?
                .clone()
        };

        let range = workbook.worksheet_range(&sheet_name)
            .map_err(|e| format!(
                "Failed to read worksheet range for sheet '{}': {}",
                sheet_name,
                e,
            ))?;

        let file = File::create(csv_file_path)
            .map_err(|e| format!(
                "Failed to create output CSV file '{}': {}",
                csv_file_path,
                e,
            ))?;

        let writer = BufWriter::new(file);
        let mut csv_writer = WriterBuilder::new()
            .delimiter(b',')
            .terminator(csv::Terminator::CRLF)
            .from_writer(writer);

        for row in range.rows() {
            let mut csv_row: Vec<String> = Vec::new();
            for cell in row.iter() {
                csv_row.push(cell.to_string());
            }

            csv_writer.write_record(&csv_row)
                .map_err(|e| format!("Failed to write CSV record: {}", e))?;
        }

        csv_writer.flush()
            .map_err(|e| format!("Failed to flush CSV writer: {}", e))?;

        Ok(())
    })();

    match result {
        Ok(_) => ConversionResult::Success,
        Err(e) => {
            let error_msg = e.to_string();
            write_error_message(
                error_message_buf,
                error_message_buf_len,
                &error_msg,
            );

            if error_msg.contains("Failed to open Excel workbook") {
                ConversionResult::ExcelReadError
            } else if error_msg.contains("No sheets found") {
                ConversionResult::NoSheetsFound
            } else if error_msg.contains("Failed to read worksheet range") {
                ConversionResult::WorksheetRangeError
            } else if error_msg.contains("Failed to create output CSV file") {
                ConversionResult::CsvFileCreateError
            } else if error_msg.contains(
                "Failed to write CSV record")
                || error_msg.contains("Failed to flush CSV writer"
            ) {
                ConversionResult::CsvWriteError
            } else if error_msg.contains("Invalid filename encoding") {
                ConversionResult::InvalidInputFilename
            } else {
                ConversionResult::OtherError
            }
        }
    }
}
