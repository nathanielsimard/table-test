# Table Test

This library aims to make table testing reliable in `Rust`. 
The problem is `assert_eq!` calling `panic!` when it fails making all the other cases untested.