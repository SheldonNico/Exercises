#ifndef TAC_TOKEN_H
#define TAC_TOKEN_H
typedef struct TOKEN_STRUCT {
    char* value;
    enum {
        TOKEN_ID,
        TOKEN_EQUALS,
        TOKEN_LPRAEN,
        TOKEN_RPRAEN,
        TOKEN_LBRACE,
        TOKEN_RBRACE,
        TOKEN_COLON,
        TOKEN_COMMA,
        TOKEN_LT,
        TOKEN_GT,
        TOKEN_ARROW_RIGHT,
        TOKEN_INT,
        TOKEN_SEMI,
        TOKEN_EOF,
    } type;
} token_T;

token_T* init_token(char* value, int type);

char* token_to_string(token_T* token);

const char* token_type_to_str(int type);

#endif
