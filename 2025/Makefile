CC=gcc
CXX=g++

all: day1 day2 day3

run: day1 day2 day3
	./day1 < day1.dat
	./day2 < day2.dat
	./day3 < day3.dat

run-day2: day2
	./day2 < day2.dat

run-day3: day3
	./day3 < day3.dat

testrun-day3: day3
	./day3 < testday3.dat

day1: day1.c
	$(CC) day1.c -o day1

day2: day2.cpp
	$(CC) -O3 -Wall day2.cpp -o day2

day3: day3.c
	$(CC) -O3 -Wall day3.c -o day3

clean:
	rm day1 day2
