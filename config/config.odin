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

Extras :: struct {
	settings: [dynamic]string,
	entries:  [dynamic]string,
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

read_extras :: proc() -> (Extras, bool) {
	extras_path := "/etc/limine/extras.conf"
	output: Extras
	if !os.exists(extras_path) {
		return output, false
	}
	extras, _ := os.read_entire_file(extras_path)
	defer delete(extras)

	it := string(extras)
	entries_start := false
	for line in strings.split_lines_iterator(&it) {
		trimmed := line
		if entries_start == false {
			trimmed = strings.trim_left(line, " ")
		}
		// Skip comments
		if strings.starts_with(trimmed, "#") {
			continue
		} else if strings.starts_with(trimmed, "/") {
			entries_start = true
		}

		if entries_start == true {
			append_elem(&output.entries, fmt.aprintf("%v\n", line))
		} else {
			append_elem(&output.settings, fmt.aprintf("%v\n", line))
		}
	}
	return output, true
}

generate_entries :: proc(kernel_list: [dynamic]Kernel, uuid := "") -> [dynamic]ConfigEntry {
	entries: [dynamic]ConfigEntry
	prefix: string
	if uuid == "" {
		prefix = "boot():"
	} else {
		prefix = fmt.aprintf("uuid(%v):", uuid)
	}
	for kernel_entry in kernel_list {
		path := fmt.aprintf("%v%v", prefix, kernel_entry.vmlinuz[5:])
		modules_path := fmt.aprintf("%v%v", prefix, kernel_entry.initramfs[5:])
		comment := fmt.aprintf("Boot %v!", kernel_entry.version)
		entry := ConfigEntry{kernel_entry.version, "linux", path, modules_path, comment}
		append_elem(&entries, entry)
	}
	return entries
}

generate_config :: proc(kernel_files: [dynamic]Kernel, uuid := "") -> string {
	entries: [dynamic]ConfigEntry
	if uuid == "" {
		entries = generate_entries(kernel_files)
	} else {
		entries = generate_entries(kernel_files, uuid = uuid)
	}
	output: [dynamic]string
	extras, ok := read_extras()
	if ok {
		for setting in extras.settings {
			append_elem(&output, setting)
		}
	}
	for entry in entries {
		entry_text := fmt.aprintfln(
			"/%v\n    protocol: %v\n    path: %v\n    module_path: %v\n    comment: %v", // Use uniform 4 spaces
			entry.name,
			entry.protocol,
			entry.path,
			entry.modules_path,
			entry.comment,
		)
		append_elem(&output, entry_text)
	}
	if ok {
		extra_entries, _ := strings.concatenate(extras.entries[:])
		append_elem(&output, extra_entries)
	}
	return strings.concatenate(output[:])
}

