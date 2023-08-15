from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="jwtsign",
    version="1.0.1",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=[],
    rust_extensions=[RustExtension("jwtsign", binding=Binding.PyO3)],
    include_package_data=True,
    zip_safe=False,
)
