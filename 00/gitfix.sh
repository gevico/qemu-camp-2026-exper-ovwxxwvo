#!/usr/bin/env bash

script_dir=$(dirname "$(realpath "${BASH_SOURCE[0]}")")
cd "${script_dir}" && cd ../ && pwd

branch="fix/rust-workspace-member-add-i2c"
commit="fix(rust): add hw/i2c to workspace members"

function git_fix() {

    echo "---------- ---------- ---------- ----------"
    # git stash
    git fetch origin
    git reset --hard origin/main
    echo "---------- ---------- ---------- ----------"
    git checkout -b "$branch"

    echo "---------- ---------- ---------- ----------"
    read -p "Press Enter to continue after fix."

    echo "---------- ---------- ---------- ----------"
    git add rust/Cargo.toml
    git commit -m "$commit"
    git push origin "$branch"

    echo "---------- ---------- ---------- ----------"
    git checkout main

    }

git_fix
