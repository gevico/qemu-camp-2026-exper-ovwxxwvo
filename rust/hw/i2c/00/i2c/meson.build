
_libi2c_bindings_inc_rs = rust.bindgen(
    args: bindgen_args_common + [
        '--allowlist-file',
        meson.project_source_root() / 'include/hw/i2c/i2c.h',
        '--blocklist-file',
        meson.project_source_root() / 'include/(block\|chardev/|exec/|hw/core/|qemu/|qom/|system/).*',
        '--opaque-type',
        'Error',
    ],
    kwargs: bindgen_kwargs,
)

_libi2c_rs = static_library(
    'i2c_rs',
    structured_sources(
        [
            'src/lib.rs',
            'src/bus.rs',
            'src/core.rs',
            'src/device.rs',
            'src/registers.rs',
            'src/bindings.rs',
            'src/test.rs',
        ],
        {'.': _libi2c_bindings_inc_rs},
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
    when: 'CONFIG_X_I2C_RUST',
    if_true: [
        declare_dependency(
            link_whole: [_libi2c_rs],
            variables: {'crate': 'i2c_rs'},
        ),
    ],
)

test(
    'rust-i2c-unit',
    executable(
        'rust-i2c-unit',
        # files('src/lib.rs', 'src/bus.rs'),
        structured_sources(
            [
                'src/lib.rs',
                'src/bus.rs',
                'src/core.rs',
                'src/device.rs',
                'src/registers.rs',
                'src/bindings.rs',
                'src/test.rs',
            ],
            {'.': _libi2c_bindings_inc_rs},
        ),
        rust_args: ['--test'],
        install: false,
        # dependencies: [common_rs]
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
    ),
    args: [
        '--test',
        '--test-threads',
        '1',
        '--format',
        'pretty',
    ],
    protocol: 'rust',
    suite: ['unit', 'rust'],
)
