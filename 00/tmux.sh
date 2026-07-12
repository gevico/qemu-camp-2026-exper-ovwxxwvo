#!/usr/bin/env bash

script_dir=$(dirname "$(realpath "${BASH_SOURCE[0]}")")
script_dir=$(dirname "$script_dir")
cd "${script_dir}" && cd ../ && pwd

tmux new-window -c "${script_dir}/rust/hw/i2c/i2c_gpio/"
tmux new-window -c "${script_dir}"
tmux new-window -c "${script_dir}/include/hw/i2c/"
tmux new-window -c "${script_dir}/hw/i2c/"
tmux new-window -c "${script_dir}/rust/hw/i2c/i2c_gpio/src/"
tmux new-window -c "${script_dir}/rust/hw/i2c/i2c_gpio/src/"
tmux new-window -c "${script_dir}/rust/hw/char/pl011/"
tmux new-window -c "${script_dir}"

