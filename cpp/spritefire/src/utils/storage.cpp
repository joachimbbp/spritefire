#include <fstream>
#include <map>
#include <string>
#include "color.h"
#include "storage.h"

std::map<std::string, RGBColor> decodeColorDb(std::string dbPath) {
    std::ifstream fin(dbPath, std::ios::binary);
    if (!fin) {
        throw std::runtime_error("Failed to open database file");
    }

    std::map<std::string, RGBColor> db;

    fin.read(reinterpret_cast<char*>(&db), sizeof(db));

    return db;
}

void printColorDb(std::string dbPath) {
    auto db = decodeColorDb(dbPath);
    for (auto const& [key, val] : db) {
        printf("key: %s, r: %d, g: %d, b: %d\n", key.c_str(), val.r, val.g, val.b);
    }
}