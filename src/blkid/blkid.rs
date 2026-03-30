#![allow(dead_code)]
#![allow(unused_variables)]
use super::ffi::*;
use libc::c_char;
use std::ffi::CStr;

pub fn evaluate_tag(token: &str, value: &str, cache: &mut Cache) -> String {
    let res: String;
    unsafe {
        let ret = blkid_evaluate_tag(token.as_ptr().cast(), value.as_ptr().cast(), cache);
        res = CStr::from_ptr(ret).to_string_lossy().into_owned();
    }
    return res;
}

pub fn evaluate_spec(spec: &str, cache: &mut Cache) -> String {
    let res: String;
    unsafe {
        let ret = blkid_evaluate_spec(spec.as_ptr().cast(), cache);
        res = CStr::from_ptr(ret).to_string_lossy().into_owned();
    }
    return res;
}

pub fn gc_cache(cache: Cache) {
    unsafe {
        blkid_gc_cache(cache);
    }
}

pub fn get_cache(cache: &mut Cache, filename: &str) -> bool {
    unsafe {
        let ret = blkid_get_cache(cache, filename.as_ptr().cast());
        ret == 0
    }
}

pub fn put_cache(cache: Cache) {
    unsafe {
        blkid_put_cache(cache);
    }
}

pub fn probe_all(cache: Cache) -> bool {
    unsafe { blkid_probe_all(cache) == 0 }
}

pub fn probe_all_removable(cache: Cache) -> bool {
    unsafe { blkid_probe_all_removable(cache) == 0 }
}

pub fn probe_all_new(cache: Cache) -> bool {
    unsafe { blkid_probe_all_removable(cache) == 0 }
}

pub fn verify(cache: Cache, dev: Dev) -> Dev {
    unsafe { return blkid_verify(cache, dev) }
}

pub fn dev_devname(dev: Dev) -> String {
    unsafe {
        CStr::from_ptr(blkid_dev_devname(dev))
            .to_string_lossy()
            .to_string()
    }
}

pub fn dev_has_tag(dev: Dev, r#type: &str, value: &str) -> bool {
    unsafe { blkid_dev_has_tag(dev, r#type.as_ptr().cast(), value.as_ptr().cast()) == 0 }
}

pub fn dev_iterate_begin(cache: Cache) {
    todo!()
}

pub fn dev_iterate_end(cache: Cache) {
    todo!()
}

pub fn dev_next(iterate: DevIterate, dev: &mut Dev) -> bool {
    todo!()
}

pub fn dev_set_search(iter: DevIterate, search_type: &str, search_value: &str) -> bool {
    todo!()
}

pub fn find_dev_with_tag(cache: Cache, r#type: &str, value: &str) -> Dev {
    unsafe { blkid_find_dev_with_tag(cache, r#type.as_ptr().cast(), value.as_ptr().cast()) }
}

pub fn get_dev(cache: Cache, devname: &str, flags: i32) -> Dev {
    unsafe { blkid_get_dev(cache, devname.as_ptr().cast(), flags) }
}

pub fn get_devname(cache: Cache, token: &str, value: &str) -> String {
    unsafe {
        CStr::from_ptr(blkid_get_devname(
            cache,
            token.as_ptr().cast(),
            value.as_ptr().cast(),
        ))
        .to_string_lossy()
        .into_owned()
    }
}

pub fn get_tag_value(cache: Cache, tagname: &str, devname: &str) -> String {
    unsafe {
        CStr::from_ptr(blkid_get_tag_value(
            cache,
            tagname.as_ptr().cast(),
            devname.as_ptr().cast(),
        ))
        .to_string_lossy()
        .into_owned()
    }
}

pub fn tag_iterate_begin(dev: Dev) -> TagIterate {
    todo!()
}

pub fn tag_iterate_end(iterate: TagIterate) {
    todo!()
}

pub fn tag_next(iterate: TagIterate) {
    todo!()
}

pub fn free_probe(pr: Probe) {
    unsafe {
        blkid_free_probe(pr);
    }
}

pub fn new_probe() -> Probe {
    unsafe { blkid_new_probe() }
}

pub fn new_probe_from_filename(filename: &str) -> Probe {
    unsafe { blkid_new_probe_from_filename(filename.as_ptr().cast()) }
}

pub fn probe_get_devno(pr: Probe) -> i32 {
    unsafe { blkid_probe_get_devno(pr) }
}

pub fn probe_get_fd(pr: Probe) -> i32 {
    unsafe { blkid_probe_get_fd(pr) }
}

pub fn probe_get_sectorsize(pr: Probe) -> u32 {
    unsafe { blkid_probe_get_sectorsize(pr) }
}

pub fn probe_get_sectors(pr: Probe) -> i64 {
    unsafe { blkid_probe_get_sectors(pr) }
}

pub fn probe_get_size(pr: Probe) -> i64 {
    unsafe { blkid_probe_get_size(pr) }
}

pub fn probe_get_offset(pr: Probe) -> i64 {
    unsafe { blkid_probe_get_offset(pr) }
}

pub fn probe_get_wholedisk_devno(pr: Probe) -> i32 {
    unsafe { blkid_probe_get_wholedisk_devno(pr) }
}

pub fn probe_is_wholedisk(pr: Probe) -> bool {
    unsafe { blkid_probe_is_wholedisk(pr) == 0 }
}

pub fn reset_probe(pr: Probe) {
    unsafe {
        blkid_reset_probe(pr);
    }
}

pub fn get_library_version() -> String {
    unsafe {
        let mut version: *const c_char = std::ptr::null();
        let mut date: *const c_char = std::ptr::null();
        blkid_get_library_version(&mut version as *mut _, &mut date as *mut _);
        return CStr::from_ptr(version).to_string_lossy().into_owned();
    }
}
