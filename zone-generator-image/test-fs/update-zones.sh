#! /bin/sh

echo "Mock update-zones.sh"

for i in `seq 1 10`; do
  echo $i
  echo stderr: $i 1>&2
  sleep 1
done
echo "Done."
