#!/bin/bash

# A script made to run epp in its actual use case, as an extension to BLANT.

# Incoming parameters
k=
n=
e=

# Expected files
BLANT="/home/wayne/pub/cs295p/blant-mp.sh"
INPUT="/extra/wayne1/preserve/cs295p/EdgePrediction/HI-union.el"
TIME="/usr/bin/time"

usage() {
  echo "Usage: epp.sh -k <k> -n <n> -e <e> [-h]"
}

while getopts ":h?k:n:e:" opt; do
  case ${opt} in
  h | \?)
    echo "A wrapper around the epp executable meant to facilitate running "
    echo "it in its intended manner: as an extension TO BLANT. This script will "
    echo "pass the parameters k and n to BLANT as expected, but also pass k "
    echo "and e to epp, as well as create a folder for the output named based "
    echo "on the parameters."
    echo ""
    usage
    exit 0
    ;;
  :)
    echo "Invalid option: $OPTARG requires an value" 1>&2
    exit 1
    ;;
  k)
    k=$OPTARG
    ;;
  n)
    n=$OPTARG
    ;;
  e)
    e=$OPTARG
    ;;
  esac
done

# shift parsed args
shift $((OPTIND - 1))

# declare remaining args to not be handled by getopts
[ "${1:-}" = "--" ] && shift

if [ -v "$k" ] | [ -v "$n" ] | [ -v "$e" ]; then
  usage
  exit 1
fi

OUT="./k$k-n$n-e$e-output"

if [ -d "$OUT" ]; then
  echo "Directory $OUT already exists."
  exit 1
else
  mkdir "$OUT"
fi

echo "Outputting to $OUT"

echo "Running..."

$TIME -v -o "$OUT"/total_time.txt sh -c "$TIME -v -o $OUT/blant_time.txt $BLANT -k$k -n$n $INPUT \
	| $TIME -v -o $OUT/epp_time.txt epp -k$k -e$e > $OUT/epp_output.txt"

echo "Done!"
