# syntax=docker/dockerfile:1
#
# Image of the coprocessor database rollback tool: the migration files, their
# down files and scripts/migrate.sh on top of the coprocessor db-migration image.
#
# The base image is the one that applied the migrations. It brings the same
# sqlx-cli build (0.7.2, rustls), psql, the AWS CLI and /prepare_database_url.sh,
# which turns DATABASE_URL into an RDS IAM token connection the way the
# migration Job does. The tool carries no coprocessor code.
#
# Build context: this directory (coprocessor-db-rollback/).
#
# DB_MIGRATION_TAG must be a tag of ghcr.io/zama-ai/fhevm/coprocessor/db-migration
# built from the same release branch as the migrations copied here.
ARG DB_MIGRATION_TAG=v0.15.1-0

FROM ghcr.io/zama-ai/fhevm/coprocessor/db-migration:${DB_MIGRATION_TAG} AS prod

COPY --chown=fhevm:fhevm migrations /app/migrations
COPY --chown=fhevm:fhevm scripts/migrate.sh /app/scripts/migrate.sh

USER fhevm:fhevm
WORKDIR /app

# Fail the build if any migration lacks a down file: an image that cannot roll
# everything back must not exist. The check also proves the base image has every
# tool the script needs.
RUN /app/scripts/migrate.sh check && sqlx --version

# Read-only by default: validates the image against the database and prints the
# migration state. A rollback has to be requested explicitly:
#   /app/scripts/migrate.sh rollback <target-version>
ENTRYPOINT ["/app/scripts/migrate.sh"]
CMD ["info"]

FROM prod AS dev
