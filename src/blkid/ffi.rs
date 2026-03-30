#![allow(dead_code)]
#![allow(unused)]
use libc::{c_char, c_int, c_uint, c_ulong, size_t};
use std::ffi::{CStr, CString};

#[repr(C)]
pub struct Cache {
    _data: [u8; 0],
}

#[repr(C)]
pub struct Dev {
    _data: [u8; 0],
}
#[repr(C)]
pub struct Probe {
    _data: [u8; 0],
}

#[repr(C)]
pub struct DevIterate {
    _data: [u8; 0],
}

#[repr(C)]
pub struct TagIterate {
    _data: [u8; 0],
}

#[repr(C)]
pub struct Partlist {
    _data: [u8; 0],
}

#[repr(C)]
pub struct Partition {
    _data: [u8; 0],
}

#[repr(C)]
pub struct Parttable {
    _data: [u8; 0],
}

#[repr(C)]
pub struct Topology {
    _data: [u8; 0],
}

#[link(name = "blkid")]
unsafe extern "C" {
    pub fn blkid_evaluate_tag(
        token: *const c_char,
        value: *const c_char,
        cache: *mut Cache,
    ) -> *mut c_char;
    pub fn blkid_evaluate_spec(spec: *const c_char, cache: *mut Cache) -> *mut c_char;

    pub fn blkid_gc_cache(cache: Cache);
    pub fn blkid_get_cache(cache: *mut Cache, filename: *const c_char) -> c_int;
    pub fn blkid_put_cache(cache: Cache);

    pub fn blkid_probe_all(cache: Cache) -> c_int;
    pub fn blkid_probe_all_removable(cache: Cache) -> c_int;
    pub fn blkid_probe_all_new(cache: Cache) -> c_int;

    pub fn blkid_verify(cache: Cache, dev: Dev) -> Dev;

    pub fn blkid_dev_devname(dev: Dev) -> *const c_char;
    pub fn blkid_dev_has_tag(dev: Dev, r#type: *const c_char, value: *const c_char) -> c_int;
    pub fn blkid_dev_iterate_begin(cache: Cache) -> DevIterate;
    pub fn blkid_dev_iterate_end(iterate: DevIterate);
    pub fn blkid_dev_next(iterate: DevIterate, dev: *mut Dev) -> c_int;
    pub fn blkid_dev_set_search(
        iter: DevIterate,
        search_type: *const c_char,
        search_value: *const c_char,
    ) -> c_int;

    pub fn blkid_find_dev_with_tag(
        cache: Cache,
        r#type: *const c_char,
        value: *const c_char,
    ) -> Dev;
    pub fn blkid_get_dev(cache: Cache, devname: *const c_char, flags: c_int) -> Dev;
    pub fn blkid_get_devname(
        cache: Cache,
        token: *const c_char,
        value: *const c_char,
    ) -> *mut c_char;
    pub fn blkid_get_tag_value(
        cache: Cache,
        tagname: *const c_char,
        devname: *const c_char,
    ) -> *mut c_char;

    pub fn blkid_tag_iterate_begin(dev: Dev) -> TagIterate;
    pub fn blkid_tag_iterate_end(iterate: TagIterate);
    pub fn blkid_tag_next(
        iterate: TagIterate,
        r#type: *mut *const c_char,
        value: *mut *const c_char,
    );

    pub fn blkid_free_probe(pr: Probe);
    pub fn blkid_new_probe() -> Probe;
    pub fn blkid_new_probe_from_filename(filename: *const c_char) -> Probe;
    pub fn blkid_probe_get_devno(pr: Probe) -> c_int;
    pub fn blkid_probe_get_fd(pr: Probe) -> c_int;
    pub fn blkid_probe_get_sectorsize(pr: Probe) -> c_uint;
    pub fn blkid_probe_get_sectors(pr: Probe) -> i64;
    pub fn blkid_probe_get_size(pr: Probe) -> i64;
    pub fn blkid_probe_get_offset(pr: Probe) -> i64;
    pub fn blkid_probe_get_wholedisk_devno(pr: Probe) -> c_int;
    pub fn blkid_probe_set_device(pr: Probe, fd: c_int, off: i64, size: i64) -> c_int;
    pub fn blkid_probe_is_wholedisk(pr: Probe) -> c_int;
    pub fn blkid_reset_probe(pr: Probe);

    pub fn blkid_do_fullprobe(pr: Probe) -> c_int;
    pub fn blkid_do_wipe(pr: Probe, dryrun: c_int) -> c_int;
    pub fn blkid_do_probe(pr: Probe) -> c_int;
    pub fn blkid_do_safeprobe(pr: Probe) -> c_int;
    pub fn blkid_probe_get_value(
        pr: Probe,
        num: c_int,
        name: *mut *const c_char,
        data: *mut *const c_char,
        len: *mut size_t,
    ) -> c_int;
    pub fn blkid_probe_has_value(pr: Probe, name: *const c_char) -> c_int;
    pub fn blkid_probe_lookup_value(
        pr: Probe,
        name: *const c_char,
        data: *mut *const c_char,
        len: *mut size_t,
    ) -> c_int;
    pub fn blkid_probe_numof_values(pr: Probe) -> c_int;

    pub fn blkid_probe_enable_superblocks(pr: Probe, enable: c_int) -> c_int;
    pub fn blkid_known_fstype(fstype: *const c_char) -> c_int;
    pub fn blkid_superblocks_get_name(
        idx: size_t,
        name: *mut *const c_char,
        usage: *mut c_int,
    ) -> c_int;
    pub fn blkid_probe_filter_superblocks_type(
        pr: Probe,
        flag: c_int,
        names: *mut *const c_char,
    ) -> c_int;
    pub fn blkid_probe_filter_superblocks_usage(pr: Probe, flag: c_int, usage: c_int) -> c_int;
    pub fn blkid_probe_invert_superblocks_filter(pr: Probe) -> c_int;
    pub fn blkid_probe_reset_superblocks_filter(pr: Probe) -> c_int;
    pub fn blkid_probe_set_superblocks_flags(pr: Probe, flags: c_int) -> c_int;
    pub fn blkid_probe_reset_filter(pr: Probe) -> c_int;
    pub fn blkid_probe_filter_types(pr: Probe, flag: c_int, names: *mut *const c_char) -> c_int;
    pub fn blkid_probe_filter_usage(pr: Probe, flag: c_int, usage: c_int) -> c_int;
    pub fn blkid_probe_invert_filter(pr: Probe) -> c_int;
    pub fn blkid_probe_set_request(pr: Probe, flags: c_int) -> c_int;

    pub fn blkid_probe_enable_partitions(pr: Probe, enable: c_int) -> c_int;
    pub fn blkid_probe_set_partition_flags(pr: Probe, flags: c_int) -> c_int;
    pub fn blkid_probe_filter_partitions_type(
        pr: Probe,
        flag: c_int,
        names: *mut *const c_char,
    ) -> c_int;
    pub fn blkid_probe_invert_partitions_filter(pr: Probe) -> c_int;
    pub fn blkid_probe_reset_partitions_filter(pr: Probe) -> c_int;
    pub fn blkid_known_pttype(pttype: *const c_char) -> c_int;

    pub fn blkid_partition_get_name(par: Partition) -> *const c_char;
    pub fn blkid_partition_get_flags(par: Partition) -> u64;
    pub fn blkid_partition_get_partno(par: Partition) -> c_int;
    pub fn blkid_partition_get_size(pr: Partition) -> i64;
    pub fn blkid_partition_get_start(pr: Partition) -> i64;
    pub fn blkid_partition_get_table(pr: Partition) -> Parttable;
    pub fn blkid_partition_get_type(pr: Partition) -> c_int;
    pub fn blkid_partition_get_type_string(pr: Partition) -> *const c_char;
    pub fn blkid_partition_get_uuid(pr: Partition) -> *const c_char;

    pub fn blkid_partition_is_extended(pr: Partition) -> c_int;
    pub fn blkid_partition_is_logical(pr: Partition) -> c_int;
    pub fn blkid_partition_is_primary(pr: Partition) -> c_int;

    pub fn blkid_partlist_get_partition(ls: Partlist, n: c_int) -> Partition;
    pub fn blkid_partlist_numof_partitions(ls: Partlist) -> c_int;
    pub fn blkid_partlist_devno_to_partition(ls: Partlist, devno: i64) -> Partition;
    pub fn blkid_partlist_get_table(ls: Partlist) -> Parttable;

    pub fn blkid_parttable_get_offset(tab: Parttable) -> i64;
    pub fn blkid_parttable_get_parent(tab: Parttable) -> Partition;
    pub fn blkid_parttable_get_type(tab: Parttable) -> *const c_char;

    pub fn blkid_probe_get_partitions(pr: Probe) -> Partlist;

    pub fn blkid_probe_enable_topology(pr: Probe, enable: c_int) -> c_int;
    pub fn blkid_probe_get_topology(pr: Probe) -> Topology;
    pub fn blkid_topology_get_alignment_offset(tp: Topology) -> c_ulong;
    pub fn blkid_topology_get_logical_sector_size(tp: Topology) -> c_ulong;
    pub fn blkid_topology_get_minimum_io_size(tp: Topology) -> c_ulong;
    pub fn blkid_topology_get_optimal_io_size(tp: Topology) -> c_ulong;
    pub fn blkid_topology_get_physical_sector_size(tp: Topology) -> c_ulong;

    pub fn blkid_encode_string(str: *const c_char, str_enc: *mut c_char, len: size_t) -> c_int;
    pub fn blkid_safe_string(str: *const c_char, str_safe: *mut c_char, len: size_t) -> c_int;

    pub fn blkid_devno_to_devname(devno: i64) -> *mut c_char;
    pub fn blkid_devno_to_wholedisk(
        dev: i64,
        diskname: *mut c_char,
        len: size_t,
        diskdevno: *mut i64,
    ) -> c_int;
    pub fn blkid_get_dev_size(fd: c_int) -> i64;
    pub fn blkid_get_library_version(
        ver_string: *mut *const c_char,
        date_string: *mut *const c_char,
    ) -> c_int;
    pub fn blkid_parse_tag_string(
        token: *const c_char,
        ret_type: *mut *const c_char,
        ret_val: *mut *const c_char,
    ) -> c_int;
    pub fn blkid_parse_version_string(ver_string: *const c_char) -> c_int;
    pub fn blkid_send_uevent(devname: *const c_char, action: *const c_char) -> c_int;
}
