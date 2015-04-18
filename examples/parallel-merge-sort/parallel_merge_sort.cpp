#include <future>
#include <iostream>
#include <string>
#include <vector>

// Merge two sorted arrays into a single sorted array.
template <typename E>
std::vector<E> merge(std::vector<E>& left, std::vector<E>& right) {
    std::vector<E> result;

    while(!left.empty() && !right.empty()) {
        if (*left.begin() <= *right.begin()) {
            result.push_back(*left.begin());
            left.erase(left.begin());;
        } else {
            result.push_back(*right.begin());
            right.erase(right.begin());
        }
    }

    // If there are any remaining elements, then push them on to the result
    // vector.
    for (auto&& element : left) {
        result.push_back(element);
    }

    for (auto&& element : right) {
        result.push_back(element);
    }

    return result;
}

// Perform a recursive mergesort.
template <typename E>
std::vector<E>  mergesort(const std::vector<E>& elements) {
    if (elements.size() <= 1) {
        return elements;
    }

    size_t middle = elements.size() / 2;
    std::vector<E> left(elements.begin(), elements.begin() + middle);
    std::vector<E> right(elements.begin() + middle, elements.end());

    // If we have fewer than 10 elements, don't spawn a new thread.
    if (elements.size() < 10) {
        left = mergesort(left);
        right = mergesort(right);
    } else {
        // Spawn a new task sorting the left side, and perform the right-side
        // merge on the same thread.
        auto leftMerge = std::async(std::launch::async, mergesort<E>, left);
        right = mergesort(right);

        // Wait for the left side to finish.
        left = leftMerge.get();
    }

    return merge(left, right);
}

int main() {
    std::vector<int> elements;

    // Parse integers from stdin.
    for (std::string line; std::getline(std::cin, line);) {
        elements.push_back(std::stoi(line));
    }

    // Perform the sort, and print sorted elements to stdout.
    elements = mergesort(elements);
    for (auto&& element : elements) {
        std::cout << element << std::endl;
    }
}
