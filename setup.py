from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(rust_extensions=[RustExtension(target="jwtsign.jwtsign", binding=Binding.PyO3)])
