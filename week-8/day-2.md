# Week 8 Day 2 Notes

Yesterday, I spent a fair amount of time working on some wasm-bindgen issues. I ended up getting a few different PR's merged! I implemented bindings to forEach for JavaScript maps, sets, and arrays, so you can now pass Rust closures as a callback parameter across the WebAssembly/JS boundary.

This is part of a larger project to add bindings for all globally available functions defined in the ECMAScript standard, which will be a promising foundation for other tools in the future. So, while it may not be immediately useful in an of itself, I do find it pretty silly and delightful that this code works now:

```rust
                #![feature(use_extern_macros, wasm_custom_section)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn sum_indices_of_evens(array: &js::Array) -> u32 {
                    let mut res = 0;
                    array.for_each(&mut |elem: JsValue, i, _| {
                        match elem.as_f64() {
                            Some(val) if val % 2. == 0. => res += i,
                            _ => { }
                        }
                    });

                    res
                }
```

I got some review notes on my Cranelift PR, and it is just about ready to be merged! I have some pattern matching details to change for the sake of consistency with the rest of the codebase, but once that's done, I should be done with all of the projects I intended to work on at RC.

What does this mean for my last few weeks? I've decided on a very silly project, which I'll be messing around with. Other than that, I might try and pair up with more people, when I'm not focusing on the job hunt.
