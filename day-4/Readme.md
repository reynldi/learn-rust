### What is Ownership

1. *Ownership* is a set of rules that governs how a Rust program manages memory since rust doesn’t have garbage collector.
2. Many programming languages don’t require you to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
3. Stack is used for static memory allocation and Heap for dynamic memory allocation, both stored in the computer's RAM
4. The stack stores values in the order it gets them and removes the values in the opposite order. referred as last-in, first-out
5. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
6. The heap is less organized: when you put data on the heap, you request a certain amount of space. Heap will be referred as a pointer.
7. For Heap case: Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

### Memory and Allocation

- String Literals will be stored in stack since it’s has known value and size and it will be compiled into final executable file.
- String literals are immutable. To make it mutable we can use `String` type
- With `String` type we can make them mutable and flexible size. So, we need to allocate an amount of memory into heap.
- Heap has unknown size, so we need to allocate them before compile time to preserve the memory for the it’s content later.

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/387f3964-fb5f-4fb6-b81b-1f29fc126546/Untitled.png)

- A `String` is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/0ace7b39-8c0c-4c21-bbaa-1f527c879181/Untitled.png)

- When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. In other words, the data representation in memory looks like Figure 4-2.
- This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as a *double free* error
- To solve this problem, we can use a `Shallow Copy` the concept of copying the pointer, length, and capacity without copying the data
- or `Deep Copy` the concept of copying the pointer, length, and capacity including copying the data
- If the value is integer. We don’t need to make a shallow/deep copy since it has fixed and known size.

### Terms

- blob: Binary large Object