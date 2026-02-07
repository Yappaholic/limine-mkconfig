package main

import "config"
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
	for fi in fis {
		_, name := filepath.split(fi.fullpath)

		if fi.is_dir {
			append_elem(&kernels_list, name)
		}
	}
	return kernels_list
}


get_boot_files :: proc(quiet := false) -> [dynamic]config.Kernel {
	kernel_list := get_installed_kernels()
	kernel_files: [dynamic]config.Kernel
	if !quiet {
		log.info("Assuming no separate /boot/efi partition")
		log.info("Assuming no separate /boot/EFI partition")
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

main :: proc() {
	Options :: struct {
		output: os.Handle `args:"pos=1,file=cw,name=o" usage:"Save config to selected path"`,
		quiet:  bool `args:"name=q" usage:"Don't ouput configure messages"`,
	}
	opt: Options
	style: flags.Parsing_Style = .Unix

	flags.parse_or_exit(&opt, os.args, style)

	check_system()
	config := config.generate_config(get_boot_files(quiet = opt.quiet))
	if opt.output != 0 {
		os.write_string(opt.output, config)
	} else {
		fmt.println(config)
	}
	delete(config)
}

