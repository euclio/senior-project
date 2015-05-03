#ifndef INT_LINKED_LIST_HPP
#define INT_LINKED_LIST_HPP

#include <memory>

class IntLinkedList {
    private:
        struct Node {
            int element;
            std::shared_ptr<Node> next;

            Node(int element);
            Node(int element, std::shared_ptr<Node> next);
        };

        std::shared_ptr<Node> headNode = nullptr;
        size_t num_nodes;

        std::shared_ptr<Node> traverse(size_t num_elements) const;
        void ensure_capacity(size_t index) const;
    public:
        int get(size_t index) const;
        void insert(size_t index, int element);
        int remove(size_t index);
        int head() const;
        int tail() const;
        size_t size() const;
};

#endif  // INT_LINKED_LIST_H
