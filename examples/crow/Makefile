all: app

app: main.cpp
	@g++ -std=c++23 -g -Wall -Wextra -pedantic \
		-I../../include \
		main.cpp -o app -lpthread

.PHONY: clean run

clean:
	@rm -rf app

run: app
	@./app
	@rm -rf app
