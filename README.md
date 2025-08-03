# CPP Libraries

This is a personal repo that contains some ready to use c++ libraries that
are already built and ready to use from various sources including my own.

## Notes for Windows
- make sure to copy the .dlls to the directory of the executable
- Makefile in examples doesn't work on windows, but build.zig does

## Libraries

### excel_to_csv
- cd ./src/excel_to_csv
- cargo build --release
- cp target/release/libexcel_to_csv.so ../../../../linux/lib
- add ./include/excel_to_csv.h and ./include/excel_to_csv.hpp to your project
- add ./linux/lib/libexcel_to_csv.so to your project

### crow
- download library from this site: https://sourceforge.net/projects/asio/
- unzip, tar, etc the file
- git clone https://github.com/CrowCpp/Crow.git
- add ./include/crow and ./include/crow.h to your project
- add ./include/asio and ./include/asio.hpp to your project

### soci
- git clone --recurse-submodules https://github.com/SOCI/soci.git
- cd soci
- mkdir build
- cd build
- cmake -G "Unix Makefiles" -DSOCI_ODBC=ON -DSOCI_TESTS=OFF -DSOCI_SHARED=ON -DCMAKE_INSTALL_PREFIX="$HOME/Coding/cpp/soci_install" ..
- cmake --build .
- cmake --install .
- add ./linux/lib/lib...so to your project
