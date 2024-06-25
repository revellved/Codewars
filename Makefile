# mk_path = $(abspath $(lastword $(MAKEFILE_LIST)))
# thisdir = $(notdir $(patsubst %/,%,$(dir $(mk_path))))
thisdir = $(notdir $(shell pwd))
thisdir_caps = `echo $(thisdir) | tr a-z A-Z`

mod_files = `git status --short | grep -e "^M" -e "^ M" | cut -f 3 -d " " | tr '\n' ' '`
rnm_files = `git status --short | grep -e "^R" -e "^ R" | cut -f 3 -d " " | tr '\n' ' '`
del_files = `git status --short | grep -e "^D" -e "^ D" | cut -f 3 -d " " | tr '\n' ' '`
add_files = `git status --short | grep -e "^A" -e "^ A" | cut -f 3 -d " " | tr '\n' ' '`

commit:
	git add .
	git commit -m "(🍩 ${thisdir_caps}) Update ${mod_files} | Rename ${rnm_files} | Added ${add_files} | Delete { ${del_files}}"
	git push --quiet &

check: 
	run = $(shell ${mod_files})
ifeq (run,)
	@echo "HELLo"
endif
