thisdir = $(notdir $(shell pwd))
thisdir_caps = $(shell echo $(thisdir) | tr a-z A-Z)

mod_files = `git status --short | grep -e "^M" -e "^ M" | cut -f 3 -d " " | tr '\n' ' '`
rnm_files = `git status --short | grep -e "^R" -e "^ R" | cut -f 3 -d " " | tr '\n' ' '`
add_files = `git status --short | grep -e "^A" -e "^ A" | cut -f 3 -d " " | tr '\n' ' '`
del_files = `git status --short | grep -e "^D" -e "^ D" | cut -f 3 -d " " | tr '\n' ' '`

update = $(shell [[ -n $(mod_files) ]] && echo "🍩 Update $(mod_files)")
rename = $(shell [[ -n $(rnm_files) ]] && echo "🍩 Rename $(mod_files)")
adding = $(shell [[ -n $(add_files) ]] && echo "🍩 Adding $(add_files)")
delete = $(shell [[ -n $(del_files) ]] && echo "🍩 Delete $(del_files)")

commit_txt = "$(thisdir_caps) $(update) $(rename) $(adding) $(delete)"

commit:
	git add .
	git commit -m $(commit_txt)
	git push --quiet &

check:
	@echo ${commit_txt}
