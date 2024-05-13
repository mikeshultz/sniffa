from packaging.version import Version
from packaging.specifiers import Specifier

import sniffa

def test_version():
    assert Specifier('>=0.0.1').contains(sniffa.version())
