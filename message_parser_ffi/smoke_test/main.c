#include <stdio.h>
// #include <stdlib.h>

#include "../message_parser.h"

int
main (int argc, char const * const argv[])
{
    char text[] = "hello **world**. go to delta.chat";
    struct TextResultForQt result = parse_to_text_result_for_qt(text, PARSING_MODE_TEXT);
    print_text_result_for_qt(&result);
    printf("result: %s", result.html);
}
