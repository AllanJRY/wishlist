## Starting the project

### Requirements
- Docker

> if you encounter any issue with some permissions denied on database creation, it may be due to wrong rights on the database directory you defined in the environnments variable (see `.env` file), you should change the right to let the container's user have all access on this directory.

### Production mode
1. If you just cloned the project, you should make a .env file from the .env.templ file, and apply any changes you want, or keep examples values (not recommended).
2. Run command: `docker compose up` in the terminal.

### Development mode
1. If you just cloned the project, you should make a .env file from the .env.templ file, and apply any changes you want, or keep examples values (not recommended).
2. Run command: `docker compose -f docker-compose.yml -f docker-compose.dev.yml  up` in the terminal.

> Important note for windows : You must run the docker commands inside a linux distribution (via WSL, for exemple), otherwise, the `cargo watch` command will not get code updates events and will not reload.
