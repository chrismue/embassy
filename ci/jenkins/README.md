# Jenkins CI Setup

This directory contains a Docker Compose configuration for running Jenkins with persistent data storage.

## Starting Jenkins

```bash
docker compose up -d
```

Jenkins will be available at `http://localhost:8080`.

The Jenkins agent port is exposed at `50000`.

## Data Persistence

All Jenkins data (configuration, jobs, build history, plugins, credentials, etc.) is stored in
a named Docker volume called `jenkins_home`. This ensures data survives container restarts and
image updates.

## Migrating Jenkins Data to Another Machine

### Backup (on the source machine)

Stop Jenkins first to ensure a consistent backup:

```bash
docker compose down
docker run --rm \
  -v jenkins_home:/jenkins_home \
  -v "$(pwd)":/backup \
  ubuntu \
  tar czf /backup/jenkins-backup.tar.gz -C / jenkins_home
```

This creates a `jenkins-backup.tar.gz` file in the current directory containing the full
Jenkins home directory including all jobs, history, plugins and configuration.

### Restore (on the target machine)

Copy `jenkins-backup.tar.gz` to the target machine, then restore the volume:

```bash
# Create the volume if it doesn't exist yet
docker volume create jenkins_home

# Restore data into the volume
docker run --rm \
  -v jenkins_home:/jenkins_home \
  -v "$(pwd)":/backup \
  ubuntu \
  tar xzf /backup/jenkins-backup.tar.gz -C /

# Start Jenkins
docker compose up -d
```

## Updating Jenkins

To update Jenkins to the latest LTS release, pull the new image and recreate the container.
Your data is preserved in the `jenkins_home` volume.

```bash
docker compose pull
docker compose up -d
```
