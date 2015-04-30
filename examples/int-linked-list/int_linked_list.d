module intlist;

import std.string : format;
import std.stdio;

class IntegerLinkedList {
    private Node headNode = null;
    private size_t numNodes = 0;

    private class Node {
        int element;
        Node next = null;

        this(int element) {
            this.element = element;
        }

        this(int element, Node next) {
            this(element);
            this.next = next;
        }
    }

    private void ensureCapacity(size_t index) const {
        if (index > this.size()) {
            throw new Exception(
                    format("index %d out of range", index));
        }
    }

    // Returns a pointer to the Node represented by the given index.
    private Node* traverse(size_t index) const {
        ensureCapacity(index);

        size_t elementsToGo = index;

        auto current = cast(Node*)&headNode;

        for (int i = 0; i < elementsToGo; i++) {
            current = &current.next;
        }

        return current;
    }

    void insert(size_t index, int element) {
        ensureCapacity(index);

        auto current = traverse(index);
        if (current !is null) {
            *current = new Node(element, *current);
        } else {
            *current = new Node(element);
        }
        ++numNodes;
    }

    int remove(size_t index) {
        ensureCapacity(index);

        auto current = traverse(index);
        int value = current.element;
        *current = current.next;
        numNodes--;
        return value;
    }

    int head() const {
        if (size() == 0) {
            throw new Exception("the list is empty");
        }

        return headNode.element;
    }

    int tail() const {
        if (size() == 0) {
            throw new Exception("the list is empty");
        }

        return traverse(numNodes - 1).element;
    }

    int get(size_t index) const {
        return traverse(index).element;
    }

    size_t size() const {
        return numNodes;
    }
}
