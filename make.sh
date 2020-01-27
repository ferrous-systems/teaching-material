#!/usr/bin/env bash
set -e

ATTRS="\
--attribute customcss=slides.css \
--attribute revealjs_width=1920 \
--attribute revealjs_height=1080 \
--attribute source-highlighter=highlightjs \
--attribute revealjs_theme=simple \
--attribute icons=font \
--attribute revealjs_history=true \
"

bundle exec asciidoctor $ATTRS ./presentations/index.adoc -o ./target/index.html
cp ./presentations/slides.css ./target/slides.css
# FIXME: this will break if there's a name collision
cp ./presentations/*/*.{svg,jpg} ./target

for d in ./presentations/*/; do
    echo $d
    bundle exec asciidoctor-revealjs $ATTRS \
       -a revealjsdir=https://cdnjs.cloudflare.com/ajax/libs/reveal.js/3.7.0 \
       $d/slides.adoc  \
       -o ./target/$(basename $d).html
done
