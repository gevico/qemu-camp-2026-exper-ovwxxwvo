
_at24c02_slave_bindings_inc_rs = rust.bindgen(
    args: bindgen_args_common + [
        '--allowlist-file',
        meson.project_source_root() / 'include/hw/i2c/at24c02_slave.h',
        '--blocklist-file',
        meson.project_source_root() / 'include/(block\|chardev/|exec/|hw/core/|qemu/|qom/|system/).*',
        '--opaque-type',
        'Error',
    ],
    kwargs: bindgen_kwargs,
)

_at24c02_slave_rs = static_library(
    'at24c02_slave',
    structured_sources(
        [
            'src/lib.rs',
            'src/device.rs',
            'src/bindings.rs',
        ],
        {'.': _at24c02_slave_bindings_inc_rs},
    ),
    override_options: ['rust_std=2021', 'build.rust_std=2021'],
    rust_abi: 'rust',
    link_with: [
        _i2c_core_rs,
        _i2c_gpio_rs,
        _system_rs,
        _util_rs,
        _qom_rs,
        _bql_rs,
        _hwcore_rs,
        _migration_rs,
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
    when: 'CONFIG_X_AT24C02_SLAVE_RUST',
    if_true: [
        declare_dependency(
            link_whole: [_at24c02_slave_rs],
            variables: {'crate': 'at24c02_slave'},
        ),
    ],
)

