
_lib_i2c_gpio_bindings_inc_rs = rust.bindgen(
    args: bindgen_args_common + [
        '--allowlist-file',
        meson.project_source_root() / 'include/hw/i2c/i2c_gpio.h',
        '--blocklist-file',
        meson.project_source_root() / 'include/(block\|chardev/|exec/|hw/core/|qemu/|qom/|system/).*',
        '--opaque-type',
        'Error',
    ],
    kwargs: bindgen_kwargs,
)

_lib_i2c_gpio_rs = static_library(
    'i2c_gpio_rs',
    structured_sources(
        [
            'src/lib.rs',
            'src/registers.rs',
            'src/device.rs',
            'src/bindings.rs',
        ],
        {'.': _lib_i2c_gpio_bindings_inc_rs},
    ),
    override_options: ['rust_std=2021', 'build.rust_std=2021'],
    rust_abi: 'rust',
    link_with: [
        _system_rs,
        _util_rs,
        _qom_rs,
        _bql_rs,
        _hwcore_rs,
        _migration_rs,
        _trace_rs,
    ],
    dependencies: [
        glib_sys_rs,
        bilge_rs,
        bilge_impl_rs,
        bits_rs,
        common_rs,
    ],
)

rust_devices_ss.add(
    when: 'CONFIG_X_I2C_GPIO_RUST',
    if_true: [
        declare_dependency(
            link_whole: [_lib_i2c_gpio_rs],
            variables: {'crate': 'i2c_gpio_rs'},
        ),
    ],
)

