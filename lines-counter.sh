#!/bin/bash

list=($(find ./* -name *.rs;))
lines=0

for((i=0; i < ${#list[@]}; i++)) {
    line=$(cat ${list[$i]} | wc -l)
    lines=$((lines + line))
}

echo $lines