
#!/bin/sh 
set -e 

cargo build --release
rm -rf picus_files
tool=target/release/midnight-extractor
export RUST_LOG=info 

constants=$(seq 1 100 | tr '\n' ,)
curve_constants=87654846584422849836571930156466438379984710599888121545025567473301233275718,45673711333516174500892987253036094404176536844955599116957274814081860440167,0

function extract {
  t=$1 
  shift 
  c=$1 
  shift
 $tool \
  --debug-comments \
  --prelude spread \
  --fail-fast \
  --type $t \
  --constants $c $@
}

# extract native $constants 
# extract byte $constants base64 --no-opt
# extract bit true 
# extract field $constants
extract biguint 0,$constants
# extract scalar 0,$constants
# extract point 0,$constants --chip ecc
# extract point "$curve_constants,$curve_constants,$curve_constants,$curve_constants,$curve_constants" --chip foreign-ecc-native 
# extract point "$curve_constants,$curve_constants,$curve_constants,$curve_constants,$curve_constants" --chip foreign-ecc-field 
