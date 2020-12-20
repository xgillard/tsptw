:

# This script runs all benchmark instances in parallel on this machine.

duration=1800

rm progress.txt

#bench=Langevin
#width=10000
#dest="${bench}_w${width}_t${duration}.txt"
#echo "Starting $bench" >> progress.txt
#./target/release/tsptw print-header > $dest
#find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest
#
bench=SolomonPotvinBengio
width=100
dest="${bench}_w${width}_t${duration}.txt"
echo "Starting $bench" >> progress.txt
./target/release/tsptw print-header > $dest
find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest


bench=Dumas
width=100
dest="${bench}_w${width}_t${duration}.txt"
echo "Starting $bench" >> progress.txt
./target/release/tsptw print-header > $dest
find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest

bench=GendreauDumasExtended
width=100
dest="${bench}_w${width}_t${duration}.txt"
echo "Starting $bench" >> progress.txt
./target/release/tsptw print-header > $dest
find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest

bench=OhlmannThomas
width=100
dest="${bench}_w${width}_t${duration}.txt"
echo "Starting $bench" >> progress.txt
./target/release/tsptw print-header > $dest
find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest

bench=SolomonPesant
width=100
dest="${bench}_w${width}_t${duration}.txt"
echo "Starting $bench" >> progress.txt
./target/release/tsptw print-header > $dest
find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest

bench=AFG
width=100
dest="${bench}_w${width}_t${duration}.txt"
echo "Starting $bench" >> progress.txt
./target/release/tsptw print-header > $dest
find ./tests/resources/$bench -type f | parallel -I% ./target/release/tsptw solve % -t 1 -w $width -d $duration >> $dest

