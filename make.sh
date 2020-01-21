#!/usr/bin/env bash
set -e

bundle exec asciidoctor-revealjs \
       -a revealjsdir=https://cdnjs.cloudflare.com/ajax/libs/reveal.js/3.7.0 \
       ./presentations/*.adoc  \
       -D ./target
