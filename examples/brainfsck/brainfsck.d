import std.algorithm;
import std.c.stdlib;
import std.range;
import std.stdio;

const int numCells = 30000;

ulong[ulong] parseLabels(File file) {
    ulong[ulong] labels;
    ulong[] parsedLabels;

    auto buf = new ubyte[](cast(size_t)file.size); file.rawRead(buf);

    foreach (ulong i, char c; buf.enumerate(1)) {
        if (c == '[') {
            parsedLabels ~= i;
        } else if (c == ']') {
            if (parsedLabels.empty) {
                stderr.writefln(
                        "Error: ']' encountered without matching ']'.");
                exit(1);
            }

            auto lastLabel = parsedLabels.back;
            parsedLabels.popBack();

            labels[i] = lastLabel;
            labels[lastLabel] = i;
        }
    }

    file.rewind();

    return labels;
}

void processOpCode(File file, ulong[ulong] labels, ref int[numCells] cells,
                   ref int cellIndex) {
    char opCode;
    file.readf("%c", &opCode);

    switch (opCode) {
        case '>':
            ++cellIndex;
            break;
        case '<':
            --cellIndex;
            break;
        case '+':
            ++cells[cellIndex];
            break;
        case '-':
            --cells[cellIndex];
            break;
        case '.':
            write(cast(char)cells[cellIndex]);
            break;
        case ',':
            stdin.readf("%c", &cells[cellIndex]);
            break;
        case '[':
            if (!cells[cellIndex]) {
                file.seek(labels[file.tell()]);
            }
            break;
        case ']':
            if (cells[cellIndex]) {
                file.seek(labels[file.tell()]);
            }
            break;
        default:
    }
}

void main(string[] args) {
    if (args.length < 2) {
        writeln("Usage: brainfsck [.bf file]");
        exit(1);
    }

    auto fileName = args[1];
    auto file = File(fileName, "r");

    int[numCells] cells;
    auto cellPointer = 0;
    auto labels = parseLabels(file);

    while (!file.eof()) {
        processOpCode(file, labels, cells, cellPointer);
    }
}
