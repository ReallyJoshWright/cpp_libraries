#include <string>
#include <print>

#include "excel_to_csv.hpp"

using std::println;
using std::string;

int main() {
    string filename = "stuff.xlsx";
    string output_filename = "output.csv";
    auto result = etc::convertExcelToCsv(filename, output_filename);
    if (!result.has_value()) {
        println("{}", result.error());
    }

    return 0;
}
