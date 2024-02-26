#!make
include .env

#setup db
setup-db:
	mysql -u${DB_USER} -p < setup/database.sql

#verify setup
verify:
	mysql -u${DB_USER} -p -e "USE rusty;SHOW TABLES\G;"

#run rusty app
rusty:
	cargo run --package rusty --bin rusty
