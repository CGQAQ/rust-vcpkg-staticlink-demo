#include <openssl/opensslv.h>

char* get_openssl_version()
{
    return OPENSSL_VERSION_TEXT;
}