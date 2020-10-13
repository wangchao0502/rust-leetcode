pname="$1"
fpath="$pname".rs
base=$(dirname "$0")

cd $base
cd ../src

cat <<EOF > $fpath
#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn $pname() {
        // codes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ${pname}_t1() {
        // let mut input = 0;

        // Solution::$pname(&mut input);
        assert_eq!(0, 0);
    }
}

EOF

# update src/main.rs
plist=""

for file in *
do
    if [ "$file" != "main.rs" ]; then
        echo $file
        plist="${plist}mod ${file/.rs/};"$'\n'
    fi
done

cat <<EOF > main.rs
$plist
fn main() {
    println!("Hello, world!");
}

EOF

echo "$pname.rs created."

