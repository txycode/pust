# rust 实现 python 解释器

## dev-v1

- 实现算数器
- 熟悉rust语法
- 熟悉tokenizer
- 练习rust生命周期

## dev-v2.01

- 实现上下文无关文法

```BNF
expr    ->  term ((PLUS | MINUS) term)*
term    ->  factor ((MUL | DIV) factor)*
factor  ->  INT | FLOAT
        ->  (PLUS | MINUS) factor
        ->  LPAR expr RPAR
```

- 构造parsor的算数器AST
- 熟悉rust引用所有权，目前ASTnode copy了所有Token，之后优化为引用

## dev-v2.02

- 拆分代码逻辑结构
- 拆分Token和Pos的依赖

## dev-v3

- 实现简单算数器的解释执行
- 还需添加测试
- 需要学习宏和范型减少重复代码

## dev-v4.01

- 实现变量声明赋值读取
- 实现power运算
  
```BNF
expr    ->  IDENTIFIER EQ expr
        ->  term ((PLUS | MINUS) term)*
term    ->  factor ((MUL | DIV) factor)*
factor  ->  (PLUS | MINUS) factor
        ->  power
power   -> atom (POW factor)*
        
atom    ->  INT | FLOAT | IDENTIFIER
        ->  LPAR expr RPAR
```
