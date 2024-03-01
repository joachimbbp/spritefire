#include <filesystem>
#include "../sprite/db.h"

int main()
{
    printf("Testing database creation");
    const std::string spriteFolderPath = "/Users/joachimpfefferkorn/Documents/GitHub/spritefire/assets/sprites_512";
    const std::string databaseSavePath = "/Users/joachimpfefferkorn/Documents/GitHub/spritefire/output";
    CreateDatabase(spriteFolderPath, databaseSavePath);
    return 1;
}