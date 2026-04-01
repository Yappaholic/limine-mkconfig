#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use libc::c_char;
use std::{
    ffi::{CStr, CString},
    path::PathBuf,
};

pub enum Status {
    Unknown = 1,
    Success = 0,
    Failure = -1,
}

impl Status {
    pub fn from(num: i32) -> Status {
        match num {
            0 => Status::Success,
            -1 => Status::Failure,
            _ => Status::Unknown,
        }
    }
}

pub fn get_library_version() -> (String, String) {
    let mut version: *const c_char = std::ptr::null_mut();
    let mut date: *const c_char = std::ptr::null_mut();
    unsafe {
        blkid_get_library_version(&mut version as *mut _, &mut date as *mut _);
        (
            CStr::from_ptr(version).to_string_lossy().to_string(),
            CStr::from_ptr(date).to_string_lossy().to_string(),
        )
    }
}

pub fn devno_to_devname(devno: dev_t) -> String {
    unsafe {
        CStr::from_ptr(blkid_devno_to_devname(devno))
            .to_string_lossy()
            .to_string()
    }
}

pub fn new_probe_from_filename(filename: &str) -> blkid_probe {
    unsafe { blkid_new_probe_from_filename(filename.as_ptr().cast()) }
}

pub fn free_probe(pr: blkid_probe) {
    unsafe { blkid_free_probe(pr) }
}

pub fn probe_is_wholedisk(pr: blkid_probe) -> bool {
    unsafe { blkid_probe_is_wholedisk(pr) == 0 }
}

pub fn probe_get_devno(pr: blkid_probe) -> dev_t {
    unsafe { blkid_probe_get_devno(pr) }
}

pub fn probe_get_wholedisk_devno(pr: blkid_probe) -> dev_t {
    unsafe { blkid_probe_get_wholedisk_devno(pr) }
}

pub fn probe_enable_partitions(pr: blkid_probe, enable: bool) -> Status {
    unsafe { Status::from(blkid_probe_enable_partitions(pr, enable.into())) }
}

pub fn probe_get_partitions(pr: blkid_probe) -> blkid_partlist {
    unsafe { blkid_probe_get_partitions(pr) }
}

pub fn partlist_devno_to_partition(ls: blkid_partlist, devno: dev_t) -> blkid_partition {
    unsafe { blkid_partlist_devno_to_partition(ls, devno) }
}

pub fn partition_get_uuid(pt: blkid_partition) -> String {
    unsafe {
        CStr::from_ptr(blkid_partition_get_uuid(pt))
            .to_string_lossy()
            .to_string()
    }
}

pub fn get_mounted_boot_device() -> String {
    let mut boot_device: String = String::from("");
    unsafe {
        let mount_file = setmntent(c"/etc/mtab".as_ptr().cast(), c"r".as_ptr().cast());
        // if mount_file.is_null() {
        //     panic!("setmntent");
        // }
        loop {
            let mount_entry = getmntent(mount_file);
            if mount_entry.is_null() {
                break;
            }
            let entry = CStr::from_ptr((*mount_entry).mnt_dir)
                .to_string_lossy()
                .to_string();
            if entry == "/boot" {
                boot_device = CStr::from_ptr((*mount_entry).mnt_fsname)
                    .to_string_lossy()
                    .to_string();
            }
        }
        endmntent(mount_file);
        boot_device
    }
}
