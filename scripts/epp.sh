#!/bin/bash

# A script made to run epp in its actual use case, as an extension to BLANT.

# Incoming parameters
k=
n=
e=8
# Assumed values
BLANT="/home/wayne/pub/cs295p/blant-mp.sh"
INPUT="/extra/wayne1/preserve/cs295p/EdgePrediction/HI-union.el"
TIME="/usr/bin/time"

usage() {
  echo "Usage: epp.sh -k <k> -n <n> [-h]"
}

while getopts ":h?k:n:e?" opt; do
  case ${opt} in
  h | \?)
    echo "A wrapper around the epp executable meant to facilitate running "
    echo "it in its intended manner: as an extension TO BLANT. This script will "
    echo "pass the parameters k and n to BLANT as expected, but also pass k "
    echo "and optionally e to epp, as well as create a folder for the output named based "
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

# if there is a "--" shift past it so
# that any args after it are indexed correctly
[ "${1:-}" = "--" ] && shift

if [ -v "$k" ] | [ -v "$n" ] | [ -v "$e" ]; then
  usage
  exit 1
fi

OUT="./k$k-n$n-e$e-output"

if [ -d "$OUT" ]; then
  echo "Directory $OUT already exists." 1>&2
  exit 1
else
  mkdir "$OUT"
fi

echo "Outputting to $OUT"

echo "Running..."

lines=$(wc -l $INPUT | cut -f1 -d' ')
range=$(bc -l <<<"1.0 /(10^$e) * $lines")

echo "Covered $lines of input with k=$k, e=$e and a range of $range" >"$OUT"/epp_stats.txt

$TIME -v -o "$OUT"/total_time.txt sh -c "$TIME -v -o $OUT/blant_time.txt $BLANT -k$k -n$n $INPUT \
	| $TIME -v -o $OUT/epp_time.txt epp -k$k -e$e > $OUT/epp_output.txt"

echo "Done!"
