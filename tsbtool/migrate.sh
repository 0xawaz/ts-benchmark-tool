#!/bin/bash
set -e

# Wait for the database to be ready
until pg_isready -h db -U "$POSTGRES_USER"; do
  echo "Waiting for database..."
  sleep 2
done

# Run migration commands
PGPASSWORD="$POSTGRES_PASSWORD" psql -h db -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f /migrations/cpu_usage.sql
PGPASSWORD="$POSTGRES_PASSWORD" psql -h db -U "$POSTGRES_USER" -d homework -c "\COPY cpu_usage FROM /migrations/cpu_usage.csv CSV HEADER"
