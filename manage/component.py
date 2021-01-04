import os, sys, sysconfig
import subprocess

import logging as log
log.basicConfig(format='[%(levelname)s] %(message)s', level=log.DEBUG)


class Component:
    def __init__(self, path):
        self.path = path
    
    def build(self, args):
        raise NotImplementedError()
    
    def test(self, args):
        raise NotImplementedError()

class Cmake(Component):
    def __init__(self, path):
        super().__init__(path)
        self.build_dir = os.path.join("build", self.path)
    
    def build(self, args):
        os.makedirs(self.build_dir, exist_ok=True)

        env = dict(os.environ)
        if args["cxx_compiler"]:
            env["CXX"] = args["cxx_compiler"]
        
        gen = []
        if "mingw" in sysconfig.get_platform().lower():
            gen = ["-G", "MinGW Makefiles"]

        cmds = [
            ["cmake"] + gen + ["../../{}".format(self.path)],
            ["make", "-j", "{}".format(os.cpu_count())]
        ]
        for cmd in cmds:
            log.info("{}> {}".format(self.build_dir, " ".join(cmd)))
            subprocess.run(cmd, env=env, cwd=self.build_dir, check=True)
    
    def test(self, args):
        subprocess.run(["ctest", "-V"] + args["args"], cwd=self.build_dir, check=True)


class Cargo(Component):
    def __init__(self, path, features=None):
        super().__init__(path)
        self.features = features
    
    def _features_args(self):
        if self.features is not None:
            return ["--features=" + ",".join(self.features)]
        return []

    def build(self, args):
        subprocess.run(["cargo", "build"] + self._features_args(), cwd=self.path, check=True)
    
    def test(self, args):
        subprocess.run(["cargo", "test"] + self._features_args(), cwd=self.path, check=True)
