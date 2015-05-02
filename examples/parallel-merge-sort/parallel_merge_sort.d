import std.algorithm;
import std.array;
import std.conv;
import std.file;
import std.parallelism : task;
import std.stdio;

T[] merge(T)(T[] left, T[] right) {
    T[] result;

    while (left.length && right.length) {
        if (left[0] < right[0]) {
            result ~= left[0];
            left = remove(left, 0);
        } else {
            result ~= right[0];
            right = remove(right, 0);
        }
    }

    foreach (ref element; left) {
        result ~= element;
    }

    foreach (ref element; right) {
        result ~= element;
    }

    return result;
}

T[] mergesort(T)(T[] elements) {
    if (elements.length <= 1) {
        return elements;
    }

    T[] left = elements[0 .. $ / 2];
    T[] right = elements[$ / 2 .. $];

    if (elements.length < 10) {
        left = mergesort(left);
        right = mergesort(right);
    } else {
        auto leftMergeTask = task!mergesort(left);
        leftMergeTask.executeInNewThread();
        right = mergesort(right);
        left = leftMergeTask.yieldForce();
    }

    return merge(left, right);
}

void main() {
    auto elements = stdin
                        .byLine()
                        .map!(to!int)
                        .array();

    elements = mergesort(elements);
    foreach (ref element; elements) {
        writeln(element);
    }
}
