#include "generic-array-list.hpp"

#include <iterator>

template <typename E>
GenericArrayList<E>::GenericArrayList() : backingArray(new E[initialSize]) {}

template<typename E>
void GenericArrayList<E>::resize(size_t newSize) {
    std::unique_ptr<E[]> newArray = new E[newSize];

    std::copy(std::begin(this->backingArray), std::end(this->backingArray),
            std::begin(newArray));

    this->backingArray = newArray;
}

template<typename E>
void GenericArrayList<E>::insert(size_t index, E element) {
    if (this->size() + 1 > this->backingArraySize) {
        this->resize(this->backingArraySize * 2);
    }

    // Shift the elements up
    for (size_t i = this->size(); i > index; --i) {
        this->backingArray[i] = this->backingArray[i - 1];
    }

    this->backingArray[index] = element;

    this->size++;
    this->backingArraySize++;
}

template<typename E>
E GenericArrayList<E>::remove(size_t index) {
    E value = this->backingArray[index];

    // Shift the elements down
    for (size_t i = index; i < this->size() - 1; ++i) {
        this->backingArray[i] = this->backingArray[i + 1];
    }

    this->size--;
    this->backingArraySize--;

    return value;
}

template<typename E>
size_t GenericArrayList<E>::size() const {
    return this->size;
}
