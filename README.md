## Starting the project

### Requirements
- Docker

> if you encounter any issue with some permissions denied on database creation, it may be due to wrong rights on the database directory you defined in the environnments variable (see `.env` file), you should change the right to let the container's user have all access on this directory.

### Before launching docker containers
If you just cloned the project, you should make a .env file from the .env.templ file, and apply any changes you want, or keep examples values (not recommended).

### Launch the containers
#### Production mode
Run command: `docker compose -f docker-compose.prod.yml up -d` in the terminal.

#### Pre-production mode
Run command: `docker compose -f docker-compose.preprod.yml up -d` in the terminal.

#### Local Development mode
2. Run command: `docker compose -f docker-compose.dev.yml up`-d in the terminal.

> Important note for windows : You must run the docker commands inside a linux distribution (via WSL, for exemple), otherwise, the `cargo watch` command will not get code updates events and will not reload.
