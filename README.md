# Pratt
A Pratt parser for a simple language (features basic math, parenthesis and nothing more) written as an exercise following [this article by the Desmos' guys](https://engineering.desmos.com/articles/pratt-parser/). This is just a parser written for didactic purposes: it does not perform any sort of evaluation of the expressions give to it; but has a pretty printer that produces basically S-expressions so, if you really want a calculator, I guess you can pipe the output of `pratt` to your loyal LISP interpreter (`echo "2 + 2" | pratt | sbcl -c` should do the trick).

Does not require any dependency, builds and runs with the standard `cargo` command `cargo build` and `cargo run`.
