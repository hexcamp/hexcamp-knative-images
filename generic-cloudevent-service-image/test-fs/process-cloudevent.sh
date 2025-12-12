#! /bin/sh

echo "Mock process-cloudevent.sh"
for i in `seq 1 10`; do
  echo $i
  echo stderr: $i 1>&2
  sleep 1
done
echo Testing timeout
set -e
timeout -v 5 sleep 10
echo "Done."
