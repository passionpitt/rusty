#setup db
setup-db:
	mysql -uroot -p < setup/database.sql

#verify setup
verify:
	mysql -uroot -p -e "USE rusty;SHOW TABLES\G;"

#run rusty app
rusty:
	cargo run --package rusty --bin rusty
