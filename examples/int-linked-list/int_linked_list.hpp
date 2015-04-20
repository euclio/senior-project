#ifndef INT_LINKED_LIST_HPP
#define INT_LINKED_LIST_HPP

#include <memory>

class IntLinkedList {
    private:
        struct Node {
            int element;
            std::shared_ptr<Node> next;

            Node(int element);
        };

        std::shared_ptr<Node> head = nullptr;
        size_t num_nodes;

        std::shared_ptr<Node> traverse(size_t num_elements) const;

    public:
        void insert(size_t index, int element);
        int remove(size_t index);
        size_t size() const;
};

#endif  // INT_LINKED_LIST_H
