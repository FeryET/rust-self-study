set shell := ["/usr/bin/bash", "-euc"]
set positional-arguments := true

md_book_version := "0.4.37"

# Define the paths for each submodule
rust_book_submodule := "resources/rust-book"
rust_by_example_submodule := "resources/rust-by-example"
exercises_submodule := "resources/100-exercises-to-learn-rust"

# Define the target directory
gen_dir := "gen"

# Recipe to update and initialize git submodules
update-submodules: 
    git submodule update --init --recursive
    cargo install mdbook --locked --version {{ md_book_version }}

[private]
_gen_book submodule book_name book_src_dir book_target_dir: update-submodules
    mkdir -p {{gen_dir}}
    cd {{ submodule }} && mdbook build "{{ book_src_dir }}"
    cp -r {{ submodule }}/{{ book_target_dir }} {{gen_dir}}/{{ book_name }}
# Recipe to prepare resources/rust-book
gen-rust-book: 
    just _gen_book "{{ rust_book_submodule }}" "rust-book" "." "book"

# Recipe to prepare resources/rustlings
gen-rust-by-example:
    just _gen_book "{{ rust_by_example_submodule }}" "rust-by-example" "." "book"

# Recipe to prepare resources/100-exercises-to-learn-rust
gen-exercises:
    just _gen_book "{{ exercises_submodule }}" "100-exercises-to-learn-rust" "book" "book/book"

# Recipe to generate all books
gen-all: gen-rust-book gen-rust-by-example gen-exercises

# Default recipe
default: gen-all