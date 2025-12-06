CC=gcc

run: day1 day2
	./day1 < day1.dat
	./day2 < day2.dat


day1: day1.c
	$(CC) day1.c -o day1

day2: day2.c
	$(CC) -Wall day2.c -lm -o day2

clean:
	rm day1 day2
