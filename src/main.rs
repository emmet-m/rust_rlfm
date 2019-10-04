use std::alloc::System;
mod bwt;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
}
