module intlisttest;

import intlist;

/// Lists have no elements when they are created.
unittest {
    auto list = new IntegerLinkedList;
    assert(list.size() == 0);
}

/// Add a single element
unittest {
    auto list = new IntegerLinkedList;
    list.insert(1, 0);
    assert(list.size() == 1);
    assert(list.head() == 0);
}

/// Remove a single element
unittest {
    auto list = new IntegerLinkedList;
    list.insert(0, 47);
    assert(list.size() == 1);
    assert(list.remove(0) == 47);
    assert(list.size() == 0);
}

/// Retrieve a single element
unittest {
    auto list = new IntegerLinkedList;
    list.insert(0, 42);
    assert(list.get(0) == 42);
    assert(list.size() == 1);
}

/// Add multiple elements
unittest {
    auto list = new IntegerLinkedList;
    list.insert(0, 1);
    list.insert(1, 2);
    assert(list.size() == 2);
    assert(list.head() == 1);
}

/// Remove multiple elements
unittest {
    auto list = new IntegerLinkedList;
    list.insert(0, 1);
    list.insert(1, 2);
    assert(list.remove(1) == 2);
    assert(list.remove(0) == 1);
}

void main() { }
