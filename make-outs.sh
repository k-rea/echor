#!/bin/zsh

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello-there1.txt
echo "Hello"   "there" > $OUTDIR/hello-there2.txt
echo -n "Hello there" > $OUTDIR/hello-there1.n.txt
echo -n "Hello"   "there" > $OUTDIR/hello-there2.n.txt
