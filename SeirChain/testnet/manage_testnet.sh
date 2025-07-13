#!/bin/bash

# Usage: ./manage_testnet.sh [up|down|logs]

case "$1" in
  up)
    echo "Starting the testnet..."
    docker-compose up -d --build
    ;;
  down)
    echo "Stopping the testnet..."
    docker-compose down
    ;;
  logs)
    echo "Tailing the logs..."
    docker-compose logs -f
    ;;
  *)
    echo "Usage: $0 {up|down|logs}"
    exit 1
esac

exit 0
