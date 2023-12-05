FROM rust:1.73.0-buster as builder

ARG APP_SRC=/var/www
# Create an empty project to pull all the dependancy, this is used to get advantage
# of the Docker cache mechanism, because that way, if an update occure in the
# project files and not in the dependancies, the dependancies pulling layer, 
# will not be run again, Docker will use his cache.
WORKDIR ${APP_SRC} 
RUN cargo new wishlist
WORKDIR ${APP_SRC}/wishlist
COPY ./Cargo.toml ./Cargo.toml 
RUN cargo build --release
RUN rm src/*.rs

# This time we copy project code.
ADD . .

# We remove the binary build from the dependancies.
RUN rm ./target/release/deps/wishlist*

# We build the final binary.
RUN cargo build --release

# Used in development environnement.
RUN cargo install cargo-watch

# We start a new layer from a fresh ubuntu installatio, this way, the builder
# stage before, which can take a lot a space will be totally removed from
# the final image build, offering a lightweight image.
FROM ubuntu:lunar
ARG APP=/var/www/wishlist

# Update ubuntu package and install useful ones
# todo: explain pulled packages and the rm cmd
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 7000

ENV TZ=Etc/UTC \ 
  APP_USER=appuser

# We create a new user which will have only permissions on the app directory.
# and we also create to application folder.
RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}


# Here is the trick, we pull only the builded binary from the builder stage.
COPY --from=builder ${APP}/target/release/wishlist ${APP}

# We change the application directory owner to appuser.
RUN chown -R $APP_USER:$APP_USER ${APP}

# We switch the user to appuser.
USER $APP_USER

# We move into the application directory.
WORKDIR ${APP}

# We run the application.
CMD ["./wishlist"]
