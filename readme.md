# rust 实现 python 解释器

## dev-v1

实现算数器

- 熟悉rust语法
- 熟悉tokenizer
- 练习rust生命周期

## dev-v2.01

实现上下文无关文法

expr    ->  term ((PLUS | MINUS) term)*
term    ->  factor ((MUL | DIV) factor)*
factor  ->  INT | FLOAT
        ->  (PLUS | MINUS) factor
        ->  LPAR expr RPAR

- 构造parsor的算数器AST
- 熟悉rust引用所有权，目前ASTnode copy了所有Token，之后优化为引用
- 拆分Token和Pos的依赖
