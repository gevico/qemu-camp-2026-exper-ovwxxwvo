
_i2c_slave_rs = static_library(
    'i2c_slave',
    structured_sources(
        [
            'src/lib.rs',
            'src/at24c02.rs',
        ],
    ),
    override_options: ['rust_std=2021', 'build.rust_std=2021'],
    rust_abi: 'rust',
    link_with: [
        _i2c_core_rs,
        # _system_rs,
        # _util_rs,
        # _qom_rs,
        # _bql_rs,
        # _hwcore_rs,
        # _migration_rs,
        # _trace_rs,
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
    when: 'CONFIG_X_I2C_SLAVE_RUST',
    if_true: [
        declare_dependency(
            link_whole: [_i2c_slave_rs],
            variables: {'crate': 'i2c_slave'},
        ),
    ],
)

