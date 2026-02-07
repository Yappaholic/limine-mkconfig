package config

import "core:fmt"
import "core:os"
import "core:strings"

ConfigEntry :: struct {
	name, protocol, path, modules_path, comment: string,
}

Kernel :: struct {
	version, vmlinuz, initramfs: string,
}

get_path :: proc() -> string {
	config_paths: []string = {
		"/boot/EFI/BOOT/limine.conf",
		"/boot/limine/limine.conf",
		"/boot/limine.conf",
		"/limine/limine.conf",
		"/limine.conf",
	}
	for path in config_paths {
		if os.exists(path) do return path
	}
	return ""
}

generate_entries :: proc(kernel_list: [dynamic]Kernel) -> [dynamic]ConfigEntry {
	entries: [dynamic]ConfigEntry
	for kernel_entry in kernel_list {
		path := fmt.aprintf("boot():%v", kernel_entry.vmlinuz[5:])
		modules_path := fmt.aprintf("boot():%v", kernel_entry.initramfs[5:])
		comment := fmt.aprintf("Boot %v!", kernel_entry.version)
		entry := ConfigEntry{kernel_entry.version, "linux", path, modules_path, comment}
		append_elem(&entries, entry)
	}
	return entries
}

generate_config :: proc(kernel_files: [dynamic]Kernel) -> string {
	entries := generate_entries(kernel_files)
	output: [dynamic]string
	for entry in entries {
		entry_text := fmt.aprintfln(
			"/%v\n\tprotocol: %v\n\tpath: %v\n\tmodules_path: %v\n\tcomment: %v",
			entry.name,
			entry.protocol,
			entry.path,
			entry.modules_path,
			entry.comment,
		)
		append_elem(&output, entry_text)
	}
	return strings.concatenate(output[:])
}

