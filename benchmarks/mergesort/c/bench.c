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

// test function 1
void merge (int *a, int n, int m) {
    int i, j, k;
    int *x = malloc(n * sizeof (int));
    for (i = 0, j = m, k = 0; k < n; k++) {
        x[k] = j == n      ? a[i++]
             : i == m      ? a[j++]
             : a[j] < a[i] ? a[j++]
             :               a[i++];
    }
    for (i = 0; i < n; i++) {
        a[i] = x[i];
    }
    free(x);
}

// test function 2
void merge_sort (int *a, int n) {
    if (n < 2)
        return;
    int m = n / 2;
    merge_sort(a, m);
    merge_sort(a + m, n - m);
    merge(a, n, m);
}


int main(int argc, char *argv[]) {    
    // getting raw merge input
    char* mergeParamRaw = argv[1];

    // removing brackets
    RemoveChars(argv[1], '[');
    RemoveChars(argv[1], ']');

    int* mergeParam = convertToIntArr(mergeParamRaw);
    int mergeParamLen = sizeof(mergeParam) / sizeof(mergeParam[0]) + 1;

    int count = atoi(argv[2]);

    // running benchmark
    for (int i = 0; i < count; i++) {
        // copying mergeParam as merge_sort is in-place
        int* mergeParamCopy = malloc(mergeParamLen * sizeof(int));
        for (int j = 0; j < mergeParamLen; j++) {
            mergeParamCopy[j] = mergeParam[j];
        }

        start_rapl();

        merge_sort(mergeParamCopy, mergeParamLen);

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
