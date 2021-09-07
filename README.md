# S

## Example
```rust
extern crate lexer;
extern crate parser;

fn main() {
    let src = r#"
        (if (< (+ 1 2 3) (* 1 2 3))
            (mod 3 2)
            "hello world")"#;

    if let Ok((s, _)) = lexer::lexer::s(src) {
        if let Ok(token) = parser::parser::parse(&lexer::s::Edge::S(s)) {
            println!("{:?}", token);
        }
    }
}

// output ↓↓↓↓↓↓↓↓
// > [Lexer          ] (:+ (1 (2 (3 nil))))
// > [Lexer          ] (:* (1 (2 (3 nil))))
// > [Lexer          ] (:< (((:+ (1 (2 (3 nil))))) (((:* (1 (2 (3 nil))))) nil)))
// > [Lexer          ] (:mod (3 (2 nil)))
// > [Lexer          ] (:if (((:< (((:+ (1 (2 (3 nil))))) (((:* (1 (2 (3 nil))))) nil)))) (((:mod (3 (2 nil)))) (""hello world"" nil))))
// > [Parser Math    ] (+ (2 (3 nil))) => Ok(6)
// > [Parser Math    ] (* (2 (3 nil))) => Ok(6)
// > [Parser BOOLEAN ] 6 < 6 => Ok(false)
// > [Parser IF      ] if(false) ((:mod (3 (2 nil)))) else ""hello world""  => ""hello world""
// > ""hello world""
```
