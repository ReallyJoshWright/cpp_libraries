all: app

app: main.cpp
	@g++ -std=c++23 -g -Wall -Wextra -pedantic \
		-I../../include \
		-L../../linux/lib \
		main.cpp -o app \
		-lexcel_to_csv \
		-Wl,-rpath=../../linux/lib

app.exe: main.cpp
	@cl /std:c++latest /EHsc /Zi /W4 /MDd \
		/I"../../include" \
		main.cpp \
		/link /LIBPATH:"../../windows/lib" excel_to_csv.lib \
		/Feapp.exe

.PHONY: clean run windows windows_run windows_clean

clean:
	@rm -rf app

run: app
	@./app
	@rm -rf app

windows: app.exe

windows_clean:
	@cmd.exe /C del app.exe

windows_run: app.exe
	@app.exe
	@cmd.exe /C del app.exe
