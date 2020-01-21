#!/usr/bin/env bash
set -e

bundle exec asciidoctor ./presentations/index.adoc -o ./target/index.html

for d in ./presentations/*/; do
    bundle exec asciidoctor-revealjs \
       -a revealjsdir=https://cdnjs.cloudflare.com/ajax/libs/reveal.js/3.7.0 \
       $d/slides.adoc  \
       -o ./target/$(basename $d).html
done
