#!/bin/bash

set -ex

GIR=UMockdev-1.0.gir
GIR_ORIG=UMockdev-1.0.orig.gir
SHA256=5267d44fae5c2d15a0fb2bcab6ee30bf5c4108518ac4e789c5eb3ff9657ec80b

cd "$(dirname "$0")"

if ! sha256sum --status --ignore-missing -c <<<"$SHA256 $GIR_ORIG"; then
  curl -L https://kojipkgs.fedoraproject.org//packages/umockdev/0.18.4/1.fc41/x86_64/umockdev-devel-0.18.4-1.fc41.x86_64.rpm \
    | bsdtar -Oxf - ./usr/share/gir-1.0/$GIR \
    > $GIR_ORIG.tmp
  sha256sum -c <<<"$SHA256 $GIR_ORIG.tmp"
  mv "$GIR_ORIG.tmp" "$GIR_ORIG"
fi

cp "$GIR_ORIG" "$GIR"
xmlstarlet ed -L \
  -u '//_:package/@name' -v umockdev-1.0 \
  -i '//_:namespace' -t attr -n shared-library -v libumockdev.so.0 \
  -u '//_:type[@name="gulong" and @c:type="gsize"]/@name' -v gsize \
  "$GIR"

if command -v delta; then
  diff=(delta --paging=never)
else
  diff=(diff -u)
fi

# Reformat the original format before diffing to only show non-formatting
# changes.
"${diff[@]}" <(xmlstarlet ed "$GIR_ORIG") "$GIR" ||:
