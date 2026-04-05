// This is for FFI bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use libc::c_char;
use std::ffi::CStr;

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
    unsafe {
        let c = format!("{}\0", filename);
        let cstr = CStr::from_bytes_until_nul(c.as_bytes()).unwrap();
        blkid_new_probe_from_filename(cstr.as_ptr())
    }
}

pub fn free_probe(pr: blkid_probe) {
    unsafe { blkid_free_probe(pr) }
}

pub fn probe_is_wholedisk(pr: blkid_probe) -> bool {
    unsafe { blkid_probe_is_wholedisk(pr) == 1 }
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
        if mount_file.is_null() {
            panic!("setmntent");
        }
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

pub fn get_device_uuid(devname: String) -> String {
    let pr = new_probe_from_filename(&devname);
    if pr.is_null() {
        eprintln!(
            "Failed to open {} for reading, did you run with sudo?",
            devname
        );
        std::process::exit(2);
    }
    if probe_is_wholedisk(pr) {
        eprintln!("can't find uuid for whole disk");
        std::process::exit(2);
    } else {
        let part_devno = probe_get_devno(pr);
        let whole_devno = probe_get_wholedisk_devno(pr);
        let wholedisk_name = devno_to_devname(whole_devno);
        free_probe(pr);
        let pr = new_probe_from_filename(&wholedisk_name);
        if pr.is_null() {
            eprintln!("Failed to create a new blkid probe for wholedisk, exiting...");
            std::process::exit(2);
        }
        match probe_enable_partitions(pr, true) {
            Status::Success => {}
            Status::Unknown => {
                eprintln!(
                    "Unknown failure to enable partitions for disk UUID discovery, exiting..."
                );
                std::process::exit(2);
            }
            Status::Failure => {
                eprintln!("Failed to enable partitions for disk UUID discovery, exiting...");
                std::process::exit(1);
            }
        }
        let partlist = probe_get_partitions(pr);
        if partlist.is_null() {
            eprintln!("Failed to get partitions list for UUID discovery, exiting...");
            std::process::exit(2);
        }
        let partition = partlist_devno_to_partition(partlist, part_devno);
        if partition.is_null() {
            eprintln!("Failed to get partition for UUID discovery, exiting...");
            std::process::exit(2);
        }
        let partition_uuid = partition_get_uuid(partition);
        free_probe(pr);
        partition_uuid
    }
}
