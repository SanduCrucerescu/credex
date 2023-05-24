# Server side of the appliation

### 1. Initialize the database
- First you need to initialize the database you can do this bu runing the command: `docket compose up .` in the root directory of the server.
- After that start the container by typing this command `docket start surrealdb`.
- The data base is empty we need to populate it by running the migrations in the utils directory  this can be done with the following command: `surreal import --conn http://127.0.0.1:8000 --user root --pass 123 --ns credex --db credex src/utils/import/import.surql`.
- If we want to save SurQL scripts the following command can be run: `surreal export --conn http://127.0.0.1:8000 --user root --pass 123 --ns credex --db credex src/utils/export/export.surql`.
- The database is fully initialised.

### 2. Starting the Server
After the database has been initialised in the root directory type: `cargo run` and the server will start.