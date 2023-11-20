#include <stdio.h>
#include <stdlib.h>

// Src: https://rosettacode.org/wiki/Base64_encode_data#C

void start_rapl();
void stop_rapl();

#include <stdio.h>
#include <unistd.h>

typedef unsigned long UL;

int main(int argc, char *argv[]) {
    int count = atoi(argv[1]);

    for (int i = 0; i < count; i++) {
        start_rapl();

        const char *alpha =	"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                    "abcdefghijklmnopqrstuvwxyz"
                    "0123456789+/";
        unsigned char c[4];
        UL u, len, w = 0;

        do {
            c[1] = c[2] = 0;

            if (!(len = read(fileno(stdin), c, 3))) break;
            u = (UL)c[0]<<16 | (UL)c[1]<<8 | (UL)c[2];

            putchar(alpha[u>>18]);
            putchar(alpha[u>>12 & 63]);
            putchar(len < 2 ? '=' : alpha[u>>6 & 63]);
            putchar(len < 3 ? '=' : alpha[u & 63]);

            if (++w == 19) w = 0, putchar('\n');
        } while (len == 3);

        if (w) putchar('\n');

        stop_rapl();
    }

	return 0;
}
