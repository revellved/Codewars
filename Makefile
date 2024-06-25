mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
current_dir := $(notdir $(patsubst %/,%,$(dir $(mkfile_path))))
current_dir_upper = `echo $(current_dir) | tr a-z A-Z`

changed_files = `git status --short | grep "^M" | cut -f 3 -d " " | tr '\n' ' '`
renamed_files = `git status --short | grep "^R" | cut -f 3 -d " " | tr '\n' ' '`
deleted_files = `git status --short | grep "^D" | cut -f 3 -d " " | tr '\n' ' '`
new_add_files = `git status --short | grep "^A" | cut -f 3 -d " " | tr '\n' ' '`

commit:
	git add .
	git commit -m "(${current_dir_upper}) Updated ${changed_files} | Renamed ${renamed_files} | Added ${new_add_files} | Deleted ${deleted_files}"
	git push --quiet &
