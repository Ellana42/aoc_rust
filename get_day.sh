day=$1
exo=$2
name_dir="day_${day}_${exo}"
mkdir -p $name_dir
cd $name_dir
cargo init 
aoc read -d $day > subject.md
aoc download -d $day
