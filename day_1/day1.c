#include "stdlib.h"
#include "stdio.h"
#include "string.h"
#include "ctype.h"

const char DIGITSTRINGS[10][7] = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
const char DIGITCHARS[10] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};

char getFirstDigitP1(const char* line) {
    int lowerPointer = 0;
    while (!isdigit(line[lowerPointer])) {
            lowerPointer++;
    }
    
    return line[lowerPointer]; 
}

char getLastDigitP1(const char* line) {
    int upperPointer = strlen(line);
    while (!isdigit(line[upperPointer])) {
        upperPointer--; 
    }
        
    return line[upperPointer];
}

int part1() {
    FILE *file = fopen("data.txt", "r");

    char currentLine[100];
    char currentDigits[3];
    currentDigits[2] = '\0';

    int sum = 0;

    while(fgets(currentLine, 100, file)) {
        currentDigits[0] = getFirstDigitP1(currentLine);
        currentDigits[1] = getLastDigitP1(currentLine);

        int currentVal = atoi(currentDigits);

        sum += currentVal;
    }

    return sum;
}

char getFirstDigitP2(const char* line) {
    int lowerPointer = 0;
    while (!isdigit(line[lowerPointer]) && lowerPointer < strlen(line)) {
        lowerPointer++;
    }
    
    int largestStringDigitPointer = strlen(line);
    int currentStringDigitIndex = 0;
    for (int i = 0; i < 10; i++) {
        char * numStringLocation = strstr(line , DIGITSTRINGS[i]);
        if (numStringLocation == NULL) {
            continue;
        }

        int numStringIndex = (numStringLocation - line);
        if (numStringIndex <= largestStringDigitPointer) {
            largestStringDigitPointer = numStringIndex;
            currentStringDigitIndex = i;
        }
    }
    
    if (largestStringDigitPointer < lowerPointer) {
        return DIGITCHARS[currentStringDigitIndex];
    }
    else {
        return line[lowerPointer];
    }
}

char * getPointerToLastStrDigit(const char * line, int digitIndex) {
    char * digitPointer = strstr(line , DIGITSTRINGS[digitIndex]);
    char * nextDigit = digitPointer;
    while (nextDigit != NULL) {
        digitPointer = nextDigit;
        nextDigit = strstr(digitPointer+sizeof(char), DIGITSTRINGS[digitIndex]);
    } 
    return digitPointer;
}

char getLastDigitP2(const char* line) {
    int upperPointer = strlen(line);
    while (!isdigit(line[upperPointer]) && upperPointer >= 0) {
        upperPointer--; 
    }

    int largestStringDigitPointer = 0;
    int currentStringDigitIndex = 0;
    for (int i = 0; i < 10; i++) {
        char * lastStrDigit = getPointerToLastStrDigit(line, i);
        if (lastStrDigit == NULL) {
            continue;
        } 

        int numStringIndex = (lastStrDigit - line);
        if (numStringIndex >= largestStringDigitPointer) {
            largestStringDigitPointer = numStringIndex;
            currentStringDigitIndex = i;
        }
    }

    if (largestStringDigitPointer > upperPointer) {
        return DIGITCHARS[currentStringDigitIndex];
    }
    else {
        return line[upperPointer];
    }
}

int part2() {
    FILE *file = fopen("data.txt", "r");

    char currentLine[100];
    char currentDigits[3];
    currentDigits[2] = '\0';

    int sum = 0;

    while(fgets(currentLine, 100, file)) {
        currentDigits[0] = getFirstDigitP2(currentLine);
        currentDigits[1] = getLastDigitP2(currentLine);

        int currentVal = atoi(currentDigits);
        sum += currentVal;
    }

    fclose(file);
    return sum;
}

int main() {
    printf("%d\n", part1());
    printf("%d\n", part2());
    return 0;
}

