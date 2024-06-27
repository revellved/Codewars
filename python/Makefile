include ../Makefile

virtualenv:
	@if [ ! -d "./bin" ]; then virtualenv .; fi

run:
	@make virtualenv
	@source ./bin/activate && python ./src/main.py

test:
	@make virtualenv
	@source ./bin/activate && python ./src/test.py


update_packages:
	pip freeze > requirements.txt

install_packages:
	pip install -r requirements.txt

u:
	make update_packages

i:
	make install_packages
