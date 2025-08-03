#pragma once

#include <cstddef>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    Success = 0,
    InvalidInputFilename = 1,
    InvalidOutputFilename = 2,
    ExcelReadError = 3,
    NoSheetsFound = 4,
    WorksheetRangeError = 5,
    CsvFileCreateError = 6,
    CsvWriteError = 7,
    OtherError = 99,
} ConversionResult;

ConversionResult convert_excel_to_csv(
    const char *filename,
    const char *output_filename,
    char *error_message_buf,
    size_t error_message_buf_len
);

#ifdef __cplusplus
}
#endif
