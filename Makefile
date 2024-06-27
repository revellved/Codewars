## With this script I fucked DRY in all the holes

ifeq ($(OS),Windows_NT)
	SHELL := C:\\Windows\System32\cmd.exe
else
	SHELL := /bin/bash
endif

thisdir = $(notdir $(shell pwd))
thisdir_caps = $(shell echo $(thisdir) | tr a-z A-Z)

mod_files = `git status --short | grep -e "^M" -e "^ M" | cut -f 3 -d " " | tr '\n' ' '`
rnm_files = `git status --short | grep -e "^R" -e "^ R" | cut -f 3 -d " " | tr '\n' ' '`
add_files = `git status --short | grep -e "^A" -e "^ A" | cut -f 3 -d " " | tr '\n' ' '`
new_files = `git status --short | grep -e "^??" -e "^ ??" | cut -f 2 -d " " | tr '\n' ' '`
del_files = `git status --short | grep -e "^D" -e "^ D" | cut -f 3 -d " " | tr '\n' ' '`

update = $(shell [[ -n $(mod_files) ]] && echo "🍩 Update $(mod_files)")
rename = $(shell [[ -n $(rnm_files) ]] && echo "🍩 Rename $(mod_files)")
adding = $(shell [[ -n $(add_files) ]] && echo "🍩 Adding $(add_files)")
newest = $(shell [[ -n $(new_files) ]] && echo "🍩 Newest $(new_files)")
delete = $(shell [[ -n $(del_files) ]] && echo "🍩 Delete $(del_files)")

commit_txt = "($(thisdir_caps)) $(update) $(rename) $(newest) $(adding) $(delete)"

commit:
	git add .
	git commit -m $(commit_txt)
	git push --quiet &

check:
	@echo ${commit_txt}
