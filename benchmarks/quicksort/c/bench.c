#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void start_rapl();
void stop_rapl();

// helper function for removing characters from string
// https://stackoverflow.com/questions/5457608/how-to-remove-the-character-at-a-given-index-from-a-string-in-c
void RemoveChars(char *s, char c)
{
    int writer = 0, reader = 0;

    while (s[reader])
    {
        if (s[reader]!=c) 
        {   
            s[writer++] = s[reader];
        }

        reader++;       
    }

    s[writer]=0;
}

// helper function for counting characters
int countChar(char* str, char c){
    int i = 0; 
    for (i=0; str[i]; str[i]==c ? i++ : *str++);
    return i;
}

// helper function for converting string to array of int (comma seperated)
int* convertToIntArr(char* str){
    int* arr = malloc(countChar(str,',') * sizeof(int));
    char* token = strtok(str, ",");
    int i = 0;
    while (token != NULL) {
        arr[i] = atoi(token);
        token = strtok(NULL, ",");
        i++;
    }
    return arr;
}

// test function 
void quicksort(int *A, int len) {
  if (len < 2) return;

  int pivot = A[len / 2];

  int i, j;
  for (i = 0, j = len - 1; ; i++, j--) {
    while (A[i] < pivot) i++;
    while (A[j] > pivot) j--;

    if (i >= j) break;

    int temp = A[i];
    A[i]     = A[j];
    A[j]     = temp;
  }

  quicksort(A, i);
  quicksort(A + i, len - i);
}


int main(int argc, char *argv[]) {    
    // getting raw merge input
    char* mergeParamRaw = argv[2];

    // removing brackets
    RemoveChars(mergeParamRaw, '[');
    RemoveChars(mergeParamRaw, ']');

    int* mergeParam = convertToIntArr(mergeParamRaw);
    int mergeParamLen = sizeof(mergeParam) / sizeof(mergeParam[0]) + 1;

    int count = atoi(argv[1]);

    // running benchmark
    for (int i = 0; i < count; i++) {
        // copying mergeParam as merge_sort is in-place
        int* mergeParamCopy = malloc(mergeParamLen * sizeof(int));
        for (int j = 0; j < mergeParamLen; j++) {
            mergeParamCopy[j] = mergeParam[j];
        }

        start_rapl();

        quicksort(mergeParamCopy, mergeParamLen);

        stop_rapl();

        // stopping compiler optimization
        if (sizeof(mergeParamCopy) < 42){
            printf("%d\n", mergeParamCopy[0]);
        }

        free(mergeParamCopy);
    }

    free(mergeParam);

    return 0;
}