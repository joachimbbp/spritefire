#pragma once

#include <map>
#include <string>
#include "color.h"

std::map<std::string, RGBColor> decodeColorDb(std::string dbPath);

void printColorDb(std::string dbPath);