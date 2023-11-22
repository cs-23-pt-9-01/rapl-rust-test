#include <iomanip>
#include <iostream>
#include <utility>

#include <openssl/bio.h>
#include <openssl/evp.h>

//#include <libnotify.h>

#ifdef __clang__
static constexpr auto COMPILER = "clang++";
#else
static constexpr auto COMPILER = "g++";
#endif

using namespace std;

class bio_string {
  BIO *bio;

public:
  bio_string(BIO *_bio) : bio(_bio) {}

  bio_string() : bio_string(BIO_new(BIO_s_mem())) {}

  bio_string(const char *pattern, size_t repeats = 1) : bio_string() {
    for (size_t i = 0; i < repeats; ++i) {
      BIO_puts(bio, pattern);
    }
  }

  bio_string(bio_string &&obj) noexcept : bio(exchange(obj.bio, nullptr)) {}

  bio_string &operator=(bio_string &&obj) {
    if (this != &obj) {
      if (bio != nullptr) {
        BIO_free_all(bio);
      }
      bio = exchange(obj.bio, nullptr);
    }
    return *this;
  }

  ~bio_string() {
    if (bio != nullptr) {
      BIO_free_all(bio);
      bio = nullptr;
    }
  }

  bio_string base64_encode() const {
    BIO *base64_filter = BIO_new(BIO_f_base64());
    BIO_set_flags(base64_filter, BIO_FLAGS_BASE64_NO_NL);
    BIO *new_bio = BIO_new(BIO_s_mem());
    BIO_set_flags(new_bio, BIO_FLAGS_BASE64_NO_NL);
    new_bio = BIO_push(base64_filter, new_bio);
    char *str;
    long length = BIO_get_mem_data(bio, &str);
    BIO_write(new_bio, str, length);
    BIO_flush(new_bio);
    return bio_string(new_bio);
  }

  bio_string base64_decode() const {
    BIO *bio, *base64_filter, *bio_out;
    char inbuf[512];
    int inlen;
    base64_filter = BIO_new(BIO_f_base64());
    BIO_set_flags(base64_filter, BIO_FLAGS_BASE64_NO_NL);
    char *str;
    long length = BIO_get_mem_data(this->bio, &str);
    bio = BIO_new_mem_buf((void *)str, length);
    bio = BIO_push(base64_filter, bio);
    bio_out = BIO_new(BIO_s_mem());
    while ((inlen = BIO_read(bio, inbuf, 512)) > 0) {
      BIO_write(bio_out, inbuf, inlen);
    }
    BIO_flush(bio_out);
    BIO_free_all(bio);
    return bio_string(bio_out);
  }

  long length() {
    char *str;
    return BIO_get_mem_data(bio, &str);
  }

  string substr(size_t pos = 0, size_t len = string::npos) const {
    char *str;
    long size = BIO_get_mem_data(bio, &str);
    return string(str + pos, len == string::npos ? size - pos : len);
  }

  friend bool operator==(const bio_string &lhs, const bio_string &rhs);
};

bool operator==(const bio_string &lhs, const bio_string &rhs) {
  return lhs.substr() == rhs.substr();
}

extern "C" {
    void start_rapl();
    void stop_rapl();
}

int main(int argc, char *argv[]) {
  const auto STR_SIZE = 131072;
  const auto TRIES = 8192;

  const bio_string str("a", STR_SIZE);
  const auto str2 = str.base64_encode();
  const auto str3 = str2.base64_decode();

  int count = std::atoi(argv[1]);

  for (int i = 0; i < count; i++) {
      start_rapl();
      stop_rapl();
  }

  //cout << fixed;
  //cout << "encode " << str.substr(0, 4) << "... to " << str2.substr(0, 4)
  //     << "...: " << s_encoded << ", " << setprecision(2) << t_encoded << endl;

  //cout << "decode " << str2.substr(0, 4) << "... to " << str3.substr(0, 4)
  //     << "...: " << s_decoded << ", " << setprecision(2) << t_decoded << endl;
}