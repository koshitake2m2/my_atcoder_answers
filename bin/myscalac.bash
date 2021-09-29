#!/bin/bash

flags=""

while test "$1" != ""; do
  case $1 in
    -*) flags="$flags $1" ;; 
    *) break ;;
  esac
  shift
done

filename=$1

shift
arguments="$@"

output_dir="generated_files_by_scalac"
mkdir $output_dir
prev_dir=`pwd`


scalac -d $output_dir $filename && cd $output_dir && scala Main
cd $prev_dir # 正しいディレクトリに戻るため.
rm -rf $output_dir
