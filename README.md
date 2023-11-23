# rustos-x86_64

Write an OS with x86_64 architecture in Rust.

## Issues

- In post 1, if you see `#![no_std]` error, please follow this issue <https://github.com/rust-lang/rust-analyzer/issues/10716> to solve.

- In post 7, if you cann't see the Race Condition in `test_println_output`, try to comment the `test_println_many`.

- In post 7, you can use command `top` to see QEMU process's %CPU.

## References

- [Writing an OS in Rust](https://os.phil-opp.com/)