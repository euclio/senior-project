#include <array>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <stack>

// The number of cells to use in memory.
const int numCells = 30000;

using cell_ptr = std::array<int, numCells>::iterator;

/*
 * Returns a map that maps where brackets should jump to.
 *
 * That is, a program containing a '[' at character 4 and a ']' at character 8
 * would return a map containing (4, 8) and (8, 4).
 */
std::map<int, int> parseLabels(std::ifstream &file) {
    std::map<int, int> labels;
    std::stack<int> parsedLabels;

    char c;
    while(file >> c) {
        if (c == '[') {
            parsedLabels.push(file.tellg());
        } else if (c == ']') {
            if (parsedLabels.empty()) {
                std::cerr <<
                    "Error: ']' encountered without matching ']'." <<
                    std::endl;
                std::exit(EXIT_FAILURE);
            }

            int lastLabel = parsedLabels.top();
            parsedLabels.pop();

            labels.insert(std::make_pair(file.tellg(), lastLabel));
            labels.insert(std::make_pair(lastLabel, file.tellg()));
        }
    }

    // We're done reading the file, so we need to set it back to the beginning.
    file.clear();
    file.seekg(std::fstream::beg);

    return labels;
}

/*
 * Runs one step of the interpreter by reading the current character the file
 * pointer is pointing at and modifies the program state accordingly.
 */
void processOpCode(std::ifstream &file, std::map<int, int> &labels,
        std::array<int, numCells> &cells, cell_ptr &pointer) {
    char opCode = file.get();

    switch (opCode) {
        case '>':
            ++pointer;
            break;
        case '<':
            --pointer;
            break;
        case '+':
            ++*pointer;
            break;
        case '-':
            --*pointer;
            break;
        case '.':
            std::putchar(*pointer);
            break;
        case ',':
            *pointer = std::getchar();
            break;
        case '[':
            if (!*pointer) file.seekg(labels.at(file.tellg()));
            break;
        case ']':
            if (*pointer) file.seekg(labels.at(file.tellg()));
            break;
    }
}

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cout << "Usage: brainfsck [.bf file]" << std::endl;
        return EXIT_FAILURE;
    }

    std::string fileName(argv[1]);
    std::ifstream file(fileName, std::ifstream::in);

    std::array<int, numCells> cells = {};
    cell_ptr pointer = cells.begin();
    std::map<int, int> labels = parseLabels(file);

    while (file.good()) {
        processOpCode(file, labels, cells, pointer);
    }

    return EXIT_SUCCESS;
}
