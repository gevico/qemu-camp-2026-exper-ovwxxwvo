#!/usr/bin/env bash

script_dir=$(dirname "$(realpath "${BASH_SOURCE[0]}")")
cd "${script_dir}" && cd ../ && pwd

function git_push() {
    echo "---------- ---------- ---------- ----------"
    git remote add upstream git@github.com:gevico/gevico-classroom-qemu-camp-2026-exper-qemu-camp-2026-exper.git
    git pull upstream main --rebase
    echo "---------- ---------- ---------- ----------"
    git add .
    echo "---------- ---------- ---------- ----------"
    git commit -m "feat: subject..."
    echo "---------- ---------- ---------- ----------"
    git push origin main
    }

git_push
