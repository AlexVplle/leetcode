#!/bin/sh

for f in *.rs; do
	crates="${crates}${next}{\"root_module\": \"$f\",\"edition\": \"2021\",\"deps\": []}"
	next=","
done

sysroot_src="$(rustc --print sysroot)/lib/rustlib/src/rust/library"

echo "{\"sysroot_src\": \"$sysroot_src\", \"crates\": [$crates]}" | jq '.' >rust-project.json
