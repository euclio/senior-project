#ifndef GENERIC_ARRAY_LIST_HPP
#define GENERIC_ARRAY_LIST_HPP

#include <memory>

template <typename E>
class GenericArrayList {
    private:
        const size_t initialSize = 10;
        std::unique_ptr<E[]> backingArray;
        size_t backingArraySize = initialSize;
        void resize(size_t newSize);

    public:
        GenericArrayList();
        void insert(size_t index, E element);
        E remove(size_t index);
        size_t size() const;
};


#endif  // GENERIC_ARRAY_LIST_HPP
