#include <stdlib.h>
#include "include/tac.h"
#include "include/lexer.h"
#include "include/io.h"
#include "include/parser.h"
#include "include/as_frontend.h"

void tac_compile(char* src) {
    lexer_T* lexer = init_lexer(src);
    parser_T* parser = init_parser(lexer);
    AST_T* root = parser_parse(parser);

    char* s = as_f(root);
    printf("%s\n", s);

/*
 *     token_T* tok = 0;
 *
 *     while ((tok = lexer_next_token(lexer))-> type != TOKEN_EOF) {
 *         printf("%s\n", token_to_string(tok));
 *     }
 */
}

void tac_compile_file(char* filename) {
    char* src = tac_read_file(filename);
    tac_compile(src);
    free(src);
}
