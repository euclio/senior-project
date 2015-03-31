#include <cctype>
#include <iostream>
#include <iterator>
#include <set>
#include <vector>

void splitSentences(std::string input) {
    if (input.length() < 3) {
        // Just assume that it's a single sentence.
        std::cout << input << std::endl;
    }

    size_t pos = 0;                     // The period we are considering.
    size_t sentence_boundary = 0;       // The start of the current sentence.

    while ((pos = input.find_first_of(".?!", pos + 1)) != std::string::npos) {
        // Periods followed by whitespace followed by a lower case letter are
        // not sentence boundaries.
        if (pos + 2 < input.length() &&
                std::isspace(input.at(pos + 1)) &&
                std::islower(input.at(pos + 2))) {
            continue;
        }

        // Periods followed by a digit with no intervening whitespace are not
        // sentence boundaries.
        if (pos + 1 < input.length() &&
                std::isdigit(input.at(pos + 1))) {
            continue;
        }

        // Periods followed by whitespace and then an upper case letter, but
        // preceded by any of a short list of titles are not sentence
        // boundaries.
        const std::set<std::string> titles = {"Mr.", "Mrs.", "Dr.", "Jr."};
        if ((pos >= 2 && titles.count(input.substr(pos - 2, 3))) ||
                (pos >= 3 && titles.count(input.substr(pos - 3, 4)))) {
            continue;
        }

        // Periods internal to a sentence of letters with no adjacent
        // whitespace are not sentence boundaries (e.g., websites).
        if (pos + 1 < input.length() &&
                std::isalpha(input.at(pos - 1)) &&
                std::isalpha(input.at(pos + 1))) {
            continue;
        }

        // Periods followed by certain kinds of punctuation (notably comma and
        // more periods) are probably not sentence boundaries.
        if (input.at(pos + 1) == ',') {
            continue;
        }

        for (; pos + 1 < input.length() && input.at(pos + 1) == '.'; ++pos) {
            // Do nothing, we are just trying to consume any periods that are
            // next to each other.
        }

        // We passed all the special conditions, so we probably have a
        // sentence.
        std::string sentence =
            input.substr(sentence_boundary, pos - sentence_boundary + 1);
        std::cout << sentence << std::endl;

        // Start the next sentence after the period and the initial space.
        sentence_boundary = pos + 2;
    }
}

int main() {
    std::istream_iterator<char> end_of_stream;
    std::istream_iterator<char> input_it (std::cin >> std::noskipws);

    std::string input(input_it, end_of_stream);
    splitSentences(input);
}
