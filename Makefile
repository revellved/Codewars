mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
current_dir := $(notdir $(patsubst %/,%,$(dir $(mkfile_path))))
current_dir_upper = `echo $(current_dir) | tr a-z A-Z`
# files = `git status --short | grep D | cut -f 3 -d " "`
files_changed = `git status --short | grep M | cut -f 3 -d " " | tr '\n' ' '`
files_deleted = `git status --short | grep D | cut -f 3 -d " " | tr '\n' ' '`
files_added = `git status --short | grep A | cut -f 3 -d " " | tr '\n' ' '`

hello:
	git add .
	git commit -m "(${current_dir_upper})\n Changed files: [ ${files_changed}].\n Added files: [ ${files_added}].\n Deleted files: [ ${files_deleted}]"
