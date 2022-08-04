#!/bin/zsh

##CHANGE THIS HERE TO YOUR DATASET FOLDER:
dataset_folder=/scratch/data
##

cargo build --release


datasets=(
$dataset_folder/artificial/random.32 \
$dataset_folder/artificial/random.16 \
$dataset_folder/artificial/random.2 \
$dataset_folder/artificial/random.4 \
$dataset_folder/artificial/random.8 \
$dataset_folder/tudocomp/pc/* \
$dataset_folder/artificial/thuemorse.20 /scratch/data/artificial/perioddoubling.20 /scratch/data/artificial/fibonacci.20 \
)


for filename in $datasets;  do
	for prefix_length in $(seq 100 20 300); do 
		# echo $prefix_length; 
		{
			IFS=$'\n' read -r -d '' CAPTURED_STDERR;
			IFS=$'\n' read -r -d '' CAPTURED_STDOUT;
		} < <((printf '\0%s\0' "$(time ./target/release/longestlyndonsubseq --filename "$filename" -p $prefix_length)" 1>&2) 2>&1)
	# echo "time: $CAPTURED_STDERR"
	# echo "str: $CAPTURED_STDOUT"
	time=$(echo $CAPTURED_STDERR | sed 's@.* \([0-9\.]\+\)s user.*@\1@')
	echo "RESULT filename=$(basename $filename) prefixlength=$prefix_length subsequencelength=$(expr length $CAPTURED_STDOUT) time=$time"
done
done
