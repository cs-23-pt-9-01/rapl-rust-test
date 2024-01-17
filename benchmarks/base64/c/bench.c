#include <assert.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef __clang__
#define COMPILER "clang"
#else
#define COMPILER "gcc"
#endif

struct str_s {
  char* buf;
  size_t size;
};

typedef struct str_s str_t;

void str_free(str_t* s) {
  free(s->buf);
}

str_t format(const char *fmt, ...) {
  va_list ap;

  // Determine required size.
  va_start(ap, fmt);
  int n = vsnprintf(NULL, 0, fmt, ap);
  va_end(ap);

  assert(n >= 0);

  size_t size = (size_t) n + 1; // One extra byte for '\0'
  char* p = malloc(size);
  assert(p != NULL);

  va_start(ap, fmt);
  n = vsnprintf(p, size, fmt, ap);
  va_end(ap);

  if (n < 0) {
    free(p);
    assert(false);
  }

  return (str_t){
    .buf = p,
    .size = size
  };
}

const char chars[] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static char decode_table[256];

size_t encode_size(size_t size) { return (size_t)(size * 4 / 3.0) + 6; }

size_t decode_size(size_t size) { return (size_t)(size * 3 / 4.0) + 6; }

void init_decode_table() {
  uint8_t ch = 0;
  do {
    char code = -1;
    if (ch >= 'A' && ch <= 'Z') {
      code = ch - 0x41;
    }
    if (ch >= 'a' && ch <= 'z') {
      code = ch - 0x47;
    }
    if (ch >= '0' && ch <= '9') {
      code = ch + 0x04;
    }
    if (ch == '+' || ch == '-') {
      code = 0x3E;
    }
    if (ch == '/' || ch == '_') {
      code = 0x3F;
    }
    decode_table[ch] = code;
  } while (ch++ < 0xFF);
}

#define next_char(x)                                                           \
  char x = decode_table[(unsigned char)*str++];                                \
  if (x < 0)                                                                   \
    return false;

int decode(size_t size, const char *str, size_t *out_size, char *output) {
  char *out = output;
  while (size > 0 && (str[size - 1] == '\n' || str[size - 1] == '\r' ||
                      str[size - 1] == '=')) {
    size--;
  }
  const char *ends = str + size - 4;
  while (true) {
    if (str > ends) {
      break;
    }
    while (*str == '\n' || *str == '\r') {
      str++;
    }

    if (str > ends) {
      break;
    }
    next_char(a);
    next_char(b);
    next_char(c);
    next_char(d);

    *out++ = (char)(a << 2 | b >> 4);
    *out++ = (char)(b << 4 | c >> 2);
    *out++ = (char)(c << 6 | d >> 0);
  }

  uint8_t mod = (ends - str + 4) % 4;
  if (mod == 2) {
    next_char(a);
    next_char(b);
    *out++ = (char)(a << 2 | b >> 4);
  } else if (mod == 3) {
    next_char(a);
    next_char(b);
    next_char(c);
    *out++ = (char)(a << 2 | b >> 4);
    *out++ = (char)(b << 4 | c >> 2);
  }

  *out = '\0';
  *out_size = out - output;
  return true;
}

inline uint32_t to_uint32_t(const char *str) {
  uint64_t n;
  memcpy(&n, str, sizeof(n));
  return n;
}

void encode(size_t size, const char *str, size_t *out_size, char *output) {
  char *out = output;
  const char *ends = str + (size - size % 3);
  uint64_t n;
  while (str != ends) {
    uint32_t n = __builtin_bswap32(to_uint32_t(str));
    *out++ = chars[(n >> 26) & 63];
    *out++ = chars[(n >> 20) & 63];
    *out++ = chars[(n >> 14) & 63];
    *out++ = chars[(n >> 8) & 63];
    str += 3;
  }
  int pd = size % 3;
  if (pd == 1) {
    n = (uint64_t)*str << 16;
    *out++ = chars[(n >> 18) & 63];
    *out++ = chars[(n >> 12) & 63];
    *out++ = '=';
    *out++ = '=';
  } else if (pd == 2) {
    n = (uint64_t)*str++ << 16;
    n |= (uint64_t)*str << 8;
    *out++ = chars[(n >> 18) & 63];
    *out++ = chars[(n >> 12) & 63];
    *out++ = chars[(n >> 6) & 63];
    *out++ = '=';
  }
  *out = '\0';
  *out_size = out - output;
}

size_t b64_encode(char *dst, const char *src, size_t src_size) {
  size_t encoded_size;
  encode(src_size, src, &encoded_size, dst);
  return encoded_size;
}

size_t b64_decode(char *dst, const char *src, size_t src_size) {
  size_t decoded_size;
  if (!decode(src_size, src, &decoded_size, dst)) {
    fputs("error when decoding", stderr);
    exit(EXIT_FAILURE);
  }
  return decoded_size;
}

void start_rapl();
void stop_rapl();

int main(int argc, char *argv[]) {
  init_decode_table();

  const int STR_SIZE = 131072;

  char str[STR_SIZE];
  memset(str, 'a', STR_SIZE);
  char str2[encode_size(STR_SIZE)];
  size_t str2_size = b64_encode(str2, str, STR_SIZE);
  char str3[decode_size(str2_size)];
  b64_decode(str3, str2, str2_size);

  int count = atoi(argv[1]);

  for (int i = 0; i < count; i++) {
      start_rapl();

      size_t str2_size = b64_encode(str2, str, STR_SIZE);
      b64_decode(str3, str2, str2_size);

      stop_rapl();
  }
}