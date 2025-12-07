CC=gcc
CXX=g++

all: day1 day2

run: day1 day2
	./day1 < day1.dat
	./day2 < day2.dat

run-day2: day2
	./day2 < day2.dat

day1: day1.c
	$(CC) day1.c -o day1

day2: day2.cpp
	$(CXX) -O3 -Wall day2.cpp -lm -o day2

clean:
	rm day1 day2
