#include "numerology.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    const char *request = "{\"operation\":\"name\",\"name\":{\"given\":\"AIIIIA\"}}";
    assert(numerology_abi_version() == 1);
    assert(strcmp(numerology_version(), "1.1.0") == 0);
    for (int i = 0; i < 1000; i++) {
        char *full = numerology_calculate_json(request);
        char *filtered = numerology_calculate_filtered_json_len((const unsigned char *)request, strlen(request));
        assert(full && filtered);
        assert(strstr(full, "normalizedName"));
        assert(!strstr(filtered, "normalizedName"));
        assert(strstr(filtered, "filtered-1.0"));
        assert(strstr(filtered, "\"root\":38"));
        assert(strstr(filtered, "\"steps\":[38,11]"));
        assert(strstr(filtered, "\"reducedValue\":2"));
        numerology_free(full);
        numerology_free(filtered);
    }
    char *null_result = numerology_calculate_filtered_json(NULL);
    assert(strstr(null_result, "NULL_INPUT"));
    numerology_free(null_result);
    char *filtered = numerology_calculate_filtered_json(request);
    assert(strstr(filtered, "filtered-1.0"));
    numerology_free(filtered);
    numerology_free(NULL);
    puts("C ABI v1: full + filtered calls passed (1000 iterations)");
    return 0;
}
