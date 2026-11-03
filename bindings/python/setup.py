import sys, platform as host
from setuptools import setup, Distribution
from wheel.bdist_wheel import bdist_wheel
class BinaryDistribution(Distribution):
    def has_ext_modules(self): return True
class PlatformWheel(bdist_wheel):
    def get_tag(self):
        _, _, platform = super().get_tag()
        if sys.platform=="darwin" and host.machine()=="arm64":platform="macosx_11_0_arm64"
        return "py3", "none", platform
setup(distclass=BinaryDistribution, cmdclass={"bdist_wheel": PlatformWheel})
