#include <iostream>

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: main.cpp <arg>\n");
        printf("Args: database, printdb, resize\n");
        return 0;
    }

    std::string mode = argv[1];

    if (mode == "database") {
        printf("Database mode\n");
    } else if (mode == "printdb") {
        printf("PrintDB mode");
    } else if (mode == "resize") {
        printf("Resize mode\n");
    } else {
        printf("Invalid mode\n");
        exit(1);
    }

    return 0;
}