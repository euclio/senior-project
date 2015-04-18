import std.algorithm;
import std.ascii : isDigit;
import std.container;
import std.file;
import std.range;
import std.stdio;
import std.uni : isAlpha, isLower, isSpace;

void splitSentences(string input) {
    if (input.length < 3) {
        // Assume that the input is a single sentence.
        writeln(input);
    }

    ptrdiff_t pos = 0;                 // The period we are considering.
    ptrdiff_t sentence_boundary = 0;   // The start of the current sentence.

    while (canFind(input[pos + 1 .. $], ".", "?", "!")) {
        pos += countUntil(input[pos + 1 .. $], ".", "?", "!") + 1;

        if (pos + 2 < input.length &&
                isSpace(input[pos + 1]) &&
                isLower(input[pos + 2])) {
            continue;
        }

        if (pos + 1 < input.length && isDigit(input[pos + 1])) {
            continue;
        }

        // Create a est of allowed titles
        auto titles = redBlackTree("Mr.", "Mrs.", "Dr.", "Jr.");
        if ((pos >= 2 && input[pos - 2 .. pos + 1] in titles) ||
                (pos >= 3 && input[pos - 3 .. pos + 1] in titles)) {
            continue;
        }

        if (pos + 1 < input.length &&
                isAlpha(input[pos - 1]) && isAlpha(input[pos + 1])) {
            continue;
        }

        if (pos + 1 < input.length && input[pos + 1] == ',') {
            continue;
        }

        while (pos + 1 < input.length && input[pos + 1] == '.') {
            pos++;
        }

        string sentence =
                input[sentence_boundary .. pos + 1];
        writeln(sentence);

        sentence_boundary = pos + 2;
    }
}

void main() {
    string input = join(stdin.byLineCopy().array());
    splitSentences(input);
}
