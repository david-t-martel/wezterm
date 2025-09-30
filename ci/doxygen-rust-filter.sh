#!/usr/bin/env bash
# Minimal Rust -> Doxygen filter.
# Strips common Rust keywords and attributes to reduce parser noise.
# Not a full parser; good enough for structural extraction.

sed -E \
  -e 's/^\s*#!\[.*\]$//' \
  -e 's/^\s*#\[.*\]$//' \
  -e 's/\b(pub(crate)?|pub|async|const|unsafe|where)\b//g' \
  -e 's/->\s*Self/-> Self/g' \
  -e 's/::/__/g' \
  -e 's/\bmod\b/namespace/' \
  -e 's/\btrait\b/class/' \
  -e 's/\bimpl\b/class/' \
  -e 's/\benum\b/class/' \
  -e 's/\bstruct\b/class/'
