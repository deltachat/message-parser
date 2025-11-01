#include <stdio.h>
// #include <stdlib.h>

#include "../message_parser.h"

int main(int argc, char const *const argv[]) {
  char text[] = "hello **world**. go to delta.chat";
  struct TextResultForQt result =
      mp_parse_to_text_result_for_qt(text, PARSING_MODE_TEXT);
  mp_print_text_result_for_qt(&result);
  printf("\nresult: %s", result.html);
  mp_free_text_result_for_qt(result);

  int emoji_count = mp_count_emojis_if_only_contains_emoji(
      "🇩🇪😅🧑‍🎨👨‍👩‍👧👩🏽‍🌾");
  printf("\nemoji count: %d, should be 5", emoji_count);

  char *first_emoji = mp_get_first_emoji("👩🏽‍🌾 Farmers");
  printf("\nfirst emoji of the string: \n%s\n", first_emoji);
  mp_free_rust_string(first_emoji);
}
