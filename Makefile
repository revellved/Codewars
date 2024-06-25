mk_path := $(abspath $(lastword $(MAKEFILE_LIST)))
thisdir := $(notdir $(patsubst %/,%,$(dir $(mk_path))))
thisdir_caps := `echo $(current_dir) | tr a-z A-Z`

mod_files := `git status --short | grep "^M" | cut -f 3 -d " " | tr '\n' ' '`
rnm_files := `git status --short | grep "^R" | cut -f 3 -d " " | tr '\n' ' '`
del_files := `git status --short | grep "^D" | cut -f 3 -d " " | tr '\n' ' '`
add_files := `git status --short | grep "^A" | cut -f 3 -d " " | tr '\n' ' '`

commit:
	git add .
	git commit -m "(${thisdir_caps}) [U] ${mod_files} | [R] ${rnm_files} | [A] ${add_files} | [D] ${del_files}"
	git push --quiet &
