#!/bin/bash

cargo build --target x86_64-pc-windows-gnu --release || exit 1

test -d release && rm -fr release
mkdir -p release/x86_64-pc-windows-gnu/tpvcbc2http/

cp target/x86_64-pc-windows-gnu/release/tpvbc2http.exe release/x86_64-pc-windows-gnu/tpvcbc2http/.
cp -r http release/x86_64-pc-windows-gnu/tpvcbc2http/.
cp README.md release/x86_64-pc-windows-gnu/tpvcbc2http/.
cp scripts/start.bat release/x86_64-pc-windows-gnu/tpvcbc2http/.
cp scripts/start_with_tpv.bat release/x86_64-pc-windows-gnu/tpvcbc2http/.

cd release/x86_64-pc-windows-gnu
zip -r -9 ../tpvcbc2http_x86_64-pc-windows-gnu.zip tpvcbc2http/
echo "Done!"