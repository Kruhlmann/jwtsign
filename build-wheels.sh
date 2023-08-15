#!/usr/bin/env sh

cd /io || exist 1

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="/root/.cargo/bin:$PATH"

for PYBIN in /opt/python/cp3*/bin; do
    "${PYBIN}/pip" install wheel setuptools-rust
    "${PYBIN}/python3" setup.py sdist bdist_wheel
done

mkdir -p wheelhouse
mv dist/*.whl wheelhouse/

for whl in wheelhouse/*.whl; do
    # auditwheel repair "$whl" --plat manylinux_2_28_x86_64 -w wheelhouse/
    echo "Contents of $whl:"
    unzip -l "$whl"
    echo "----------------------"
done
