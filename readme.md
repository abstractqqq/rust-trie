# My Rust Implementation of Trie

What is a Trie?
[https://en.wikipedia.org/wiki/Trie]

This is a slightly more customized Trie than the textbook definition.

All tests are included in main. (I know... I am too lazy to do unit testing...)

Most implementations are standard, except for a few tweaks that makes it suitable for simple 'word completion' and 'word look up' tasks. (See the 'suggest' method. See main for examples.)

I could have sacrificed more memory to store a Set of inserted values, but I don't like that idea. That's why get_string method becomes more complicated, but it is a fun programming exercise.

This is by no means an optimized implementation. I am still a rust noob. Probably doing a lot of stupid stuff here. Any suggestions are welcome. Any optimization is highly appreciated. Please let me know if there is any bug.

I am both loving and emotionally damaged by this ******* programming language......