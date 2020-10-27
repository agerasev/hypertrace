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
    def __init__(self, path, test="test"):
        super().__init__(path)
        self.build_dir = os.path.join("build", self.path)
        self.test_exec = test
    
    def build(self, args):
        os.makedirs(self.build_dir, exist_ok=True)

        env = dict(os.environ)
        if args["cxx_compiler"]:
            env["CXX"] = args["cxx_compiler"]
        
        defs = {}
        if args["cxx_use_pch"]:
            defs["USE_PCH"] = "ON"
        defargs = ["-D{}={}".format(k, v) for k, v in defs.items()]

        gen = []
        if "mingw" in sysconfig.get_platform().lower():
            gen = ["-G", "MinGW Makefiles"]

        cmds = [
            ["cmake"] + gen + ["../../{}".format(self.path)] + defargs,
            ["make", "-j", "{}".format(os.cpu_count())]
        ]
        for cmd in cmds:
            log.info("{}> {}".format(self.build_dir, " ".join(cmd)))
            subprocess.run(cmd, env=env, cwd=self.build_dir, check=True)
    
    def test(self, args):
        subprocess.run(["./" + self.test_exec] + args["args"], cwd=self.build_dir, check=True)


class Cargo(Component):
    def __init__(self, path):
        super().__init__(path)
    
    def build(self, args):
        raise NotImplementedError()
    
    def test(self, args):
        raise NotImplementedError()


components = [
    ("rstd", Cmake("rstd", test="rstd_test")),
    ("core", Cmake("core")),
    ("dyntype", Cargo("dyntype")),
]
