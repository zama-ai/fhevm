build must guarantee everything up to the full build
- clean first (because build === re-build- because i want to be 1000% sure)
- check vendor
- sync vendor
- check prettier
- check forge:fmt
- run eslint
- run forge:lint
- actual ts build
- actual solididty/forge build

test
- all tests are run on top of an existing build

rebuild_and_test:
- maximum stuff: clean + build + test

