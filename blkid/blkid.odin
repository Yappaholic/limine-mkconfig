package blkid

cache :: distinct rawptr
dev :: distinct rawptr
dev_iterate :: distinct rawptr
tag_iterate :: distinct rawptr
probe :: distinct rawptr
dev_t :: distinct int
loff_t :: distinct i64
partlist :: distinct rawptr
partition :: distinct rawptr
parttable :: distinct rawptr
DebugMask :: distinct enum {
	FULL_DEBUG = 0xffff,
}

foreign import blkid "system:libblkid.so.1"
@(link_prefix = "blkid_")
@(default_calling_convention = "c")
foreign blkid {
	// Tag
	evaluate_tag :: proc(token, value: cstring, cache: ^cache) -> cstring ---
	evaluate_spec :: proc(spec: cstring, cache: ^cache) -> cstring ---
	// Cache
	gc_cache :: proc(cache: cache) ---
	get_cache :: proc(cache: ^cache, filename: ^cstring) -> int ---
	put_cache :: proc(cache: cache) ---
	probe_all :: proc(cache: cache) -> int ---
	probe_all_removable :: proc(cache: cache) -> int ---
	probe_all_new :: proc(cache: cache) -> int ---
	verify :: proc(cache: cache, dev_: dev) -> dev ---
	// Search and iterate
	dev_devname :: proc(dev_: dev) -> cstring ---
	dev_has_tag :: proc(dev_: dev, type, value: cstring) -> int ---
	dev_iterate_begin :: proc(cache: cache) -> dev_iterate ---
	dev_iterate_end :: proc(iterate: dev_iterate) ---
	dev_next :: proc(iterate: dev_iterate, dev: ^dev) -> int ---
	dev_get_search :: proc(iter: dev_iterate, search_type, search_value: cstring) -> int ---
	find_dev_with_tag :: proc(cache_: cache, type, value: cstring) -> dev ---
	get_dev :: proc(cache_: cache, devname: cstring, flags: int) -> dev ---
	get_devname :: proc(cache_: cache, token, value: cstring) -> cstring ---
	get_tag_value :: proc(cache_: cache, tagname, devname: cstring) -> cstring ---
	tag_iterate_begin :: proc(dev_: dev) -> tag_iterate ---
	tag_iterate_end :: proc(iterate: tag_iterate) ---
	tag_next :: proc(iterate: tag_iterate, type, value: ^cstring) -> int ---
	// Initialization
	init_debug :: proc(mask: DebugMask) ---
	// Low-level probing
	free_probe :: proc(pr: probe) ---
	new_probe :: proc() -> probe ---
	new_probe_from_filename :: proc(filename: cstring) -> probe ---
	probe_get_devno :: proc(pr: probe) -> dev_t ---
	probe_get_fd :: proc(pr: probe) -> int ---
	probe_get_offset :: proc(pr: probe) -> loff_t ---
	probe_get_sectors :: proc(pr: probe) -> loff_t ---
	probe_get_sectorsize :: proc(pr: probe) -> uint ---
	probe_get_size :: proc(pr: probe) -> loff_t ---
	probe_get_wholedisk_devno :: proc(pr: probe) -> dev_t ---
	probe_is_wholedisk :: proc(pr: probe) -> int ---
	probe_set_device :: proc(pr: probe, fd: int, off, size: loff_t) -> int ---
	probe_step_back :: proc(pr: probe) -> int ---
	reset_probe :: proc(pr: probe) ---
	// Low-level tags
	do_fullprobe :: proc(pr: probe) -> int ---
	do_wipe :: proc(pr: probe, dryrun: int) -> int ---
	do_probe :: proc(pr: probe) -> int ---
	do_safeprobe :: proc(pr: probe) -> int ---
	probe_get_value :: proc(pr: probe, num: int, name, data: ^cstring, len: ^uint) -> int ---
	probe_has_value :: proc(pr: probe, name: cstring) -> int ---
	probe_lookup_value :: proc(pr: probe, name: cstring, data: ^cstring, len: ^uint) -> int ---
	probe_numof_values :: proc(pr: probe) -> int ---
	// Superblocks probing
	probe_enable_superblocks :: proc(pr: probe, enable: int) -> int ---
	known_fstype :: proc(fstype: cstring) -> int ---
	superblocks_get_name :: proc(idx: uint, name: ^cstring, usage: ^int) -> int ---
	probe_filter_superblocks_type :: proc(pr: probe, flag: int, names: []cstring) -> int ---
	probe_filter_superblocks_usage :: proc(pr: probe, flag, usage: int) -> int ---
	probe_invert_superblocks_filter :: proc(pr: probe) -> int ---
	probe_reset_superblocks_filter :: proc(pr: probe) -> int ---
	probe_set_superblocks_flags :: proc(pr: probe, flags: int) -> int ---
	probe_reset_filter :: proc(pr: probe) -> int ---
	probe_filter_types :: proc(pr: probe, flag: int, names: []cstring) -> int ---
	probe_filter_usage :: proc(pr: probe, flag, usage: int) -> int ---
	probe_invert_filter :: proc(pr: probe) -> int ---
	probe_set_request :: proc(pr: probe, flags: int) -> int ---
	// Partitions probing

}

