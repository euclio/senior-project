#include "generic-array-list.hpp"

#include <iterator>

template <typename E>
GenericArrayList<E>::GenericArrayList() : backingArray(new E[initialSize]) {}

template<typename E>
void GenericArrayList<E>::resize(size_t newSize) {
    std::unique_ptr<E[]> newArray = new E[newSize];

    std::copy(std::begin(backingArray), std::end(backingArray),
            std::begin(newArray));

    backingArray = newArray;
}

template<typename E>
void GenericArrayList<E>::insert(size_t index, E element) {
    if (size() + 1 > backingArraySize) {
        resize(backingArraySize * 2);
    }

    // Shift the elements up
    for (size_t i = size(); i > index; --i) {
        backingArray[i] = backingArray[i - 1];
    }

    backingArray[index] = element;

    numElements++;
}

template<typename E>
E GenericArrayList<E>::remove(size_t index) {
    E value = backingArray[index];

    // Shift the elements down
    for (size_t i = index; i < size() - 1; ++i) {
        backingArray[i] = backingArray[i + 1];
    }

    numElements--;

    return value;
}

template<typename E>
size_t GenericArrayList<E>::size() const {
    return numElements;
}

template<typename E>
E GenericArrayList<E>::head() const {
    return backingArray[0];
}

template<typename E>
E GenericArrayList<E>::tail() const {
    return backingArray[numElements];
}
