all: app

app: main.cpp
	@g++ -std=c++23 -g -Wall -Wextra -pedantic \
		-I../../include \
		-L../../linux/lib \
		main.cpp -o app \
		-lexcel_to_csv \
		-Wl,-rpath=../../linux/lib

.PHONY: clean run

clean:
	@rm -rf app

run: app
	@./app
	@rm -rf app
