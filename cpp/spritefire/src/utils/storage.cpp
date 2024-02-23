#include "storage.h"

#include <fstream>
#include <map>
#include <string>

#include "color.h"

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
    for (auto i = db.begin(); i != db.end(); i++) {
        printf("key: %s, r: %d, g: %d, b: %d\n", i->first.c_str(), i->second.r, i->second.g, i->second.b);
    }
}