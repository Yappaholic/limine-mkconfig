package main

when ODIN_OS != .Linux {
	#panic("This package is only supported for linux")
}

import "blkid"
import "config"
import "core:c"
import "core:flags"
import "core:fmt"
import "core:os"
import "core:path/filepath"
import "core:strings"
import "log"

VMLINUZ_PREFIX :: "vmlinuz-"
INITRAMFS_PREFIX :: "initramfs-"


check_system :: proc() {
	os_release, ok := os.read_entire_file("/etc/os-release", context.allocator)
	if !ok {
		log.error("Could not read /etc/os-release")
	}
	defer delete(os_release, context.allocator)

	it := string(os_release)
	for line in strings.split_lines_iterator(&it) {
		if strings.starts_with(line, "ID") {
			os_name := line[3:]
			if os_name == "nixos" do log.error("Not implemented for non-FHS env")
		}
	}
}

get_installed_kernels :: proc() -> [dynamic]string {
	modules_path := "/lib/modules"

	f, err := os.open(modules_path)
	defer os.close(f)

	if err != os.ERROR_NONE {
		log.error("Could not open /lib/modules for reading")
	}

	fis: []os.File_Info
	defer os.file_info_slice_delete(fis)

	fis, err = os.read_dir(f, -1)
	if err != os.ERROR_NONE {
		log.error("Could not read directory", err, exit_code = 2)
	}

	kernels_list: [dynamic]string
	// Reverse search to get from highest kernel version to lowest
	for i := len(fis) - 1; i >= 0; i -= 1 {
		fi := fis[i]
		_, name := filepath.split(fi.fullpath)

		if fi.is_dir {
			append_elem(&kernels_list, name)
		}
	}
	return kernels_list
}


get_boot_files :: proc(quiet := false, separate_efi := false) -> [dynamic]config.Kernel {
	kernel_list := get_installed_kernels()
	kernel_files: [dynamic]config.Kernel
	if !quiet {
		if !separate_efi {
			log.info("Assuming no separate /boot/efi partition")
			log.info("Assuming no separate /boot/EFI partition")
		} else {
			log.info("Assuming separate /boot/efi partition")
			log.info("Assuming separate /boot/EFI partition")
		}
	}
	for kernel in kernel_list {
		kernel_union: config.Kernel
		vmlinuz := fmt.aprintf("/boot/%v%v", VMLINUZ_PREFIX, kernel)
		initramfs := fmt.aprintf("/boot/%v%v.img", INITRAMFS_PREFIX, kernel)
		if !os.exists(vmlinuz) {
			if !quiet do log.warning("No kernel binary found for kernel", kernel, ", skipping...")
			continue
		}
		kernel_union.version = kernel
		kernel_union.vmlinuz = vmlinuz
		if !os.exists(initramfs) {
			if !quiet do log.warning("No initramfs found for kernel", kernel)
			kernel_union.initramfs = ""
		} else {
			kernel_union.initramfs = initramfs
		}
		append_elem(&kernel_files, kernel_union)
	}
	return kernel_files
}

get_mounted_boot_uuid :: proc(devname: string) -> string {
	pr := blkid.new_probe_from_filename(strings.clone_to_cstring(devname))
	defer blkid.free_probe(pr)
	if pr == nil {
		log.error("Failed to probe /boot mounted partition", devname, "did you run with sudo?")
	}
	partition_uuid: cstring
	if blkid.probe_is_wholedisk(pr) == 1 {
		log.error("Can't find UUID for whole disk", devname)
	} else {
		partition_devno := blkid.probe_get_devno(pr)
		wholedisk_devno := blkid.probe_get_wholedisk_devno(pr)
		wholedisk_name := blkid.devno_to_devname(wholedisk_devno)
		// Close previous probe
		blkid.free_probe(pr)
		// Start probing whole disk
		pr = blkid.new_probe_from_filename(wholedisk_name)
		blkid.probe_enable_partitions(pr, 1)
		partlist := blkid.probe_get_partitions(pr)
		partition := blkid.partlist_devno_to_partition(partlist, partition_devno)
		partition_uuid = blkid.partition_get_uuid(partition)
	}
	return strings.clone_from_cstring(partition_uuid)
}

get_mounted_boot_device :: proc() -> string {
	mount_file: ^c.FILE
	mount_entry: ^blkid.Mntent
	filename: cstring = "/proc/mounts" // Should be mostly available

	mount_file = blkid.setmntent(filename, "r")
	defer blkid.endmntent(mount_file)
	if mount_file == nil {
		log.error("setmntent")
	}
	boot_device: string
	for {
		mount_entry = blkid.getmntent(mount_file)
		if mount_entry == nil do break
		if mount_entry.dir == "/boot" {
			boot_device = strings.clone_from_cstring(mount_entry.fsname)
		}
	}
	return boot_device
}

main :: proc() {
	Options :: struct {
		output:       os.Handle `args:"pos=1,file=cw,name=o" usage:"Save config to selected path"`,
		separate_efi: bool `args:"pos=3,name=e" usage:"Assume separate /boot/efi partition"`,
		overwrite:    bool `args:"pos=2,name=O" usage:"Overwrite existing configuration, ignores -o"`,
		quiet:        bool `args:"name=q" usage:"Don't ouput configure messages"`,
	}
	opt: Options
	style: flags.Parsing_Style = .Unix

	flags.parse_or_exit(&opt, os.args, style)

	check_system()
	cfg: string
	if opt.separate_efi {
		boot_device := get_mounted_boot_device()
		uuid := get_mounted_boot_uuid(boot_device)
		cfg = config.generate_config(
			get_boot_files(quiet = opt.quiet, separate_efi = opt.separate_efi),
			uuid = uuid,
		)
	} else {
		cfg = config.generate_config(get_boot_files(quiet = opt.quiet))
	}

	if opt.overwrite && opt.output != 0 {
		log.error("-O and -o cannot be used at the same time")
	}
	if opt.overwrite {
		config_path := config.get_path()
		if config_path == "" do log.error("Could not find existing configuration")
		ok := os.write_entire_file(config_path, transmute([]u8)cfg)
		if !ok do log.error("Failed to overwrite existing config file at", config_path, ", did you run with sudo?")
		log.info("Overwrote existing configuration at", config_path)
	} else if opt.output != 0 {
		os.write_string(opt.output, cfg)
	} else {
		fmt.println(cfg)
	}
	delete(cfg)
}

