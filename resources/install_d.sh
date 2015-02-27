#!/usr/bin/env bash
#
# Script to install the D compiler, runtime, and standard library from a
# specific release.

D_RELEASE="v2.066.1"
set -e

mkdir d && cd d
declare -a repos=( dmd druntime phobos )
for repo in ${repos[@]}; do
    git clone "git://github.com/D-Programming-Language/${repo}.git" && cd "${repo}"
    git checkout ""
    cd ..
done

cd dmd/src
make -f posix.mak -j5 MODEL=64
cd ../../druntime
make -f posix.mak -j5 MODEL=64 DMD=../dmd/src/dmd
cd ../phobos
make -f posix.mak -j5 MODEL=64 DMD=../dmd/src/dmd
cd ..

# We have to tell the D compiler where the standard library is.
cat >dmd/src/dmd.conf <<EOL
[Environment]

DFLAGS=-I${PWD}/phobos -I${PWD}/druntime/import -L-L${PWD}/phobos/generated/linux/release/64 -L--no-warn-search-mismatch -L--export-dynamic
EOL
echo
echo "========================================================================"
echo "D installed successfully!"
echo "You probably want to add dmd to your path with the following command: "
echo
echo "export PATH=\$PATH:${PWD}/dmd/src/ >> ~/.bashrc"
echo "========================================================================"
