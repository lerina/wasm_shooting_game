# USAGE:
# ./build.sh
#
# wasm-pack args before --
# cargo args after --
# i.e:
# ./build.sh --debug -- --offline
wasm-pack build --target web --no-typescript --out-dir www/pkg $@

