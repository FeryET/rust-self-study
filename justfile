set shell := ["/usr/bin/bash", "-euc"]
set positional-arguments := true

md_book_version := "0.4.37"

# Define the paths for each submodule
rust_book_dir := "resources/rust-book"
rust_by_example_dir := "resources/rust-by-example"
exercises_dir := "resources/100-exercises-to-learn-rust"

# Define the target directory
gen_dir := "gen"

install:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v mdbook; then
        mdbook_tar=$(mktemp)
        curl -fSL 'https://github.com/rust-lang/mdBook/releases/download/v0.4.43/mdbook-v0.4.43-x86_64-unknown-linux-gnu.tar.gz' -o $mdbook_tar
        tar -xvf $mdbook_tar -C ~/bin
    fi

update:
    #!/usr/bin/env bash
    set -euxo pipefail
    echo "updating subtress..."
    for subtree_dir in "{{ rust_book_dir }}" "{{ rust_by_example_dir}}" "{{ exercises_dir }}"; do
        case $subtree_dir in
            "{{ rust_book_dir }}")
            url=https://github.com/rust-lang/book.git
            branch="main"
            ;;
            "{{ rust_by_example_dir }}")
            url=https://github.com/rust-lang/rust-by-example
            branch="master"
            ;;
            "{{ exercises_dir }}")
            url="https://github.com/mainmatter/100-exercises-to-learn-rust.git"
            branch="main"
            ;;
            *)
            echo "Bad option"
            exit 1
            ;;
        esac
        if [ ! -d $subtree_dir ]; then
            git subtree add --prefix "$subtree_dir" "$url" "$branch" --squash
        fi
        git subtree pull --prefix "$subtree_dir" "$url" "$branch" --squash
    done
    echo "finished updating subtress."


[private]
_gen_book submodule book_name book_src_dir book_target_dir:
    mkdir -p {{gen_dir}}
    cd {{ submodule }} && mdbook build "{{ book_src_dir }}"
    cp -r {{ submodule }}/{{ book_target_dir }} {{gen_dir}}/{{ book_name }}
# Recipe to prepare resources/rust-book
gen-rust-book:
    just _gen_book "{{ rust_book_dir }}" "rust-book" "." "book"

# Recipe to prepare resources/rustlings
gen-rust-by-example:
    just _gen_book "{{ rust_by_example_dir }}" "rust-by-example" "." "book"

# Recipe to prepare resources/100-exercises-to-learn-rust
gen-exercises:
    just _gen_book "{{ exercises_dir }}" "100-exercises-to-learn-rust" "book" "book/book"

# Recipe to generate all books
gen-all: gen-rust-book gen-rust-by-example gen-exercises


update-and-gen-all: update gen-all

# Default recipe
default: gen-all