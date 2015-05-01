module genericarraylist;

import std.algorithm : copy;

class ArrayList(T) {
    private T[] backingArray;
    private size_t theSize;

    this() {
        const auto initialSize = 10;
        backingArray = new T[10];
    }

    private void resize(size_t newSize) {
        T[] newArray = new T[newSize];
        copy(newArray, backingArray);
        backingArray = newArray;
    }

    void insert(size_t index, T element) {
        if (size() + 1 > backingArray.length) {
            resize(backingArray.length * 2);
        }

        copy(backingArray[index + 1 .. $], backingArray[index .. $ - 1]);
        backingArray[index] = element;

        theSize++;
    }

    T remove(size_t index) {
        T value = backingArray[index];

        copy(backingArray[index .. $-1], backingArray[index + 1.. $]);

        theSize--;

        return value;
    }

    size_t size() {
        return theSize;
    }

    T head() {
        return backingArray[0];
    }

    T tail() {
        return backingArray[theSize - 1];
    }

    T get(size_t index) {
        return backingArray[index];
    }
}
