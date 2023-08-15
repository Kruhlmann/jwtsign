from distutils import util

from setuptools import setup
from setuptools_rust import Binding, RustExtension

version: dict[str, str] = {}
version_file_path = util.convert_path("src/version.py")
with open(version_file_path) as version_file:
    exec(version_file.read(), version)  # noqa: S102 DUO105, WPS421

setup(
    rust_extensions=[RustExtension(target="jwtsign", binding=Binding.PyO3)],
    version=version["__version__"],
)
