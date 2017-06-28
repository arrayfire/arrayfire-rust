#!/bin/bash

cargo doc

if [ -d "./target/doc/arrayfire" ]; then
    # If cargo doc(generates ./target/doc directory) has been run
    cp ./doc/external_docs.css ./target/doc/

    rustdoc "./doc/getting_started.md" --markdown-css "../rustdoc.css" --markdown-css "../main.css" --markdown-css "../normalize.css" --markdown-css "../external_docs.css" -o "./target/doc/arrayfire/"
    rustdoc "./doc/array_and_matrix_manipulation.md" --markdown-css "../rustdoc.css" --markdown-css "../main.css" --markdown-css "../normalize.css" --markdown-css "../external_docs.css" --markdown-no-toc -o "./target/doc/arrayfire/"
    rustdoc "./doc/vectorization.md" --markdown-css "../rustdoc.css" --markdown-css "../main.css" --markdown-css "../normalize.css" --markdown-css "../external_docs.css" --markdown-no-toc -o "./target/doc/arrayfire/"
    rustdoc "./doc/indexing.md" --markdown-css "../rustdoc.css" --markdown-css "../main.css" --markdown-css "../normalize.css" --markdown-css "../external_docs.css" -o "./target/doc/arrayfire/"
    rustdoc "./doc/configuring_arrayfire_environment.md" --markdown-css "../rustdoc.css" --markdown-css "../main.css" --markdown-css "../normalize.css" --markdown-css "../external_docs.css" -o "./target/doc/arrayfire/"
fi
