#!/bin/bash

if [ $# -ne 1 ]
then
    echo "引数がありません"
    exit 1
fi

curl $1 | ./target/debug/rikai sample/ocaml.json > tmp/${1##*/}.ml
