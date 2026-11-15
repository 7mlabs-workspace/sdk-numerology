#ifndef SEVENMLABS_NUMEROLOGY_H
#define SEVENMLABS_NUMEROLOGY_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
uint32_t numerology_abi_version(void);
/* Borrowed static string, do not free. */
const char *numerology_version(void);
/* Preferred bounded API, supports any UTF-8 bytes including malformed JSON containing NUL. */
char *numerology_calculate_json_len(const unsigned char *input, size_t len);
/* Input must be valid NUL-terminated UTF-8. Output must be freed exactly once. */
char *numerology_calculate_json(const char *input);
/* Opt-in filtered-1.0 output; same request grammar, bounds and numerology_free ownership. */
char *numerology_calculate_filtered_json_len(const unsigned char *input, size_t len);
char *numerology_calculate_filtered_json(const char *input);
void numerology_free(char *output);
#ifdef __cplusplus
}
#endif
#endif
