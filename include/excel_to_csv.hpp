#pragma once

#include <expected>
#include <cstddef>
#include <format>
#include <string>

#include "excel_to_csv.h"

namespace etc {
    inline std::expected<void, std::string> convertExcelToCsv(std::string filename, std::string output_filename) {
        const char *filename_str = filename.c_str();
        const char *output_filename_str = output_filename.c_str();
        const size_t error_message_buffer_size = 512;
        char error_message_buffer[error_message_buffer_size];
        error_message_buffer[0] = '\0';

        ConversionResult result = convert_excel_to_csv(filename_str, output_filename_str, error_message_buffer, error_message_buffer_size);

        if (result == Success) {
            return {};
        }

        std::string error_message;
        if (error_message_buffer[0] != '\0') {
            error_message = std::string(error_message_buffer);
        } else {
            error_message = "No specific error message returned";
        }

        switch (result) {
            case InvalidInputFilename:
                return std::unexpected(std::format("Error: Invalid input filename or encoding. {}", error_message));
            case InvalidOutputFilename:
                return std::unexpected(std::format("Error: Invalid output filename or encoding. {}", error_message));
            case ExcelReadError:
                return std::unexpected(std::format("Error: Could not open or read the Excel file. {}", error_message));
            case NoSheetsFound:
                return std::unexpected(std::format("Error: No sheets found in the Excel workbook. {}", error_message));
            case WorksheetRangeError:
                return std::unexpected(std::format("Error: Failed to read data range from worksheet. {}", error_message));
            case CsvFileCreateError:
                return std::unexpected(std::format("Error: Failed to create the output CSV file (permissions?). {}", error_message));
            case CsvWriteError:
                return std::unexpected(std::format("Error: Failed to write data to the CSV file. {}", error_message));
            case OtherError:
                return std::unexpected(std::format("Error: An unexpected error occurred in Rust. {}", error_message));
            default:
                return std::unexpected(std::format("Error: Unknown error code {}. {}", static_cast<int>(result), error_message));
        }
    }
}
