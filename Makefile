thisdir = $(notdir $(shell pwd))
thisdir_caps = `echo $(thisdir) | tr a-z A-Z`

mod_files = `git status --short | grep -e "^M" -e "^ M" | cut -f 3 -d " " | tr '\n' ' '`
rnm_files = `git status --short | grep -e "^R" -e "^ R" | cut -f 3 -d " " | tr '\n' ' '`
add_files = `git status --short | grep -e "^A" -e "^ A" | cut -f 3 -d " " | tr '\n' ' '`
del_files = `git status --short | grep -e "^D" -e "^ D" | cut -f 3 -d " " | tr '\n' ' '`

commit_txt = '(${thisdir_caps}) $(shell [[ -n $(mod_files) ]] && echo "🍩 Update $(mod_files)") $(shell [[ -n $(rnm_files) ]] && echo "🍩 Rename $(mod_files)") $(shell [[ -n $(add_files) ]] && echo "🍩 Adding $(add_files)") $(shell [[ -n $(del_files) ]] && echo "🍩 Delete $(del_files)")'

commit:
	git add .
	git commit -m $(commit_txt)
	git push --quiet &

check:
	@echo ${commit_txt}
