#include "include/as_frontend.h"
#include <stdlib.h>
#include <string.h>

char* as_f_compound(AST_T* ast) {
    char* value = calloc(1, sizeof(char));
    for (int i = 0; i < (int) ast->children->size; i++) {
        AST_T* child_ast = (AST_T*) ast->children->items[i];
        char* next_value = as_f(child_ast);
        value = realloc(value, (strlen(next_value) + 1) * sizeof(char));
        strcat(value, next_value);
    }
    return value;
}

char* as_f_assignment(AST_T* ast) {
    const char* template = ".global %s\n"
        "%s:\n";
    char* s = calloc(strlen(template)+1, sizeof(char));
    strcpy(s, template);

    AST_T* as_val = ast->value;
    char* as_val_val = as_f(as_val->value);

    s = realloc(s, (strlen(s) + strlen(as_val_val) + 1) * sizeof(char));
    strcat(s, as_val_val);

    return s;
}

char* as_f_variable(AST_T* ast) {

}

char* as_f_call(AST_T* ast) {
    if (strcmp(ast->name, "return") == 0) {

    }
}

char* as_f_int(AST_T* ast) {

}

char* as_f(AST_T* ast) {
    char* value = calloc(1, sizeof(char));
    char* next_value = 0;
    switch (ast->type) {
        case AST_COMPOUND: next_value = as_f_compound(ast); break;
        case AST_ASSIGNMENT: next_value = as_f_assignment(ast); break;
        case AST_VARIABLE: next_value = as_f_variable(ast); break;
        case AST_CALL: next_value = as_f_call(ast); break;
        case AST_INT: next_value = as_f_int(ast); break;
        default: printf("[As Fontend]: No frontend for AST of type `%d`\n", ast->type); exit(1); break;
    }

    value = realloc(value, (strlen(next_value) + 1) * sizeof(char));
    strcat(value, next_value);

    return value;
}
