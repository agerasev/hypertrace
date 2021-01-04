import os, sys
import shutil
import subprocess
import argparse

import logging as log
log.basicConfig(format='[%(levelname)s] %(message)s', level=log.DEBUG)

from manage import *


class Handler:
    def __init__(self):
        self.parser = self._create_parser()

    def _create_parser(self):
        parser = argparse.ArgumentParser()
        return parser

    def _handle_args(self, args):
        return args

    def _run(self, args):
        raise NotImplementedError()

    def __call__(self, argv):
        args = vars(self.parser.parse_args(argv))
        args = self._handle_args(args)
        self._run(args)


class LoadHandler(Handler):
    def __init__(self):
        super().__init__()

    def _create_parser(self):
        parser = argparse.ArgumentParser(
            description="Download submodules",
            parents=[super()._create_parser()], add_help=False,
        )
        return parser
    
    def _handle_args(self, args):
        args = super()._handle_args(args)
        return args

    def _run(self, args):
        subprocess.run(["git", "submodule", "update", "--init", "--recursive"], check=True)


class BuildHandler(LoadHandler):
    def __init__(self):
        super().__init__()

    def _create_parser(self):
        parser = argparse.ArgumentParser(
            description="Build the components",
            parents=[super()._create_parser()], add_help=False,
        )
        parser.add_argument(
            "-c", "--components", action="append", nargs="+", type=str,
            choices=list(zip(*components))[0], default=[],
            help="Select components to build. All by default."
        )
        parser.add_argument(
            "--cxx-compiler", type=str, default=None,
            help="Select C++ compiler to build. If not specified then system-default will be used."
        )
        return parser
    
    def _handle_args(self, args):
        args = super()._handle_args(args)

        if not args["components"]:
            args["components"] = list(zip(*components))[0]
        else:
            args["components"] = [k for l in args["components"] for k in l]
        log.info("Components selected: {}".format(args["components"]))

        return args

    def _run(self, args):
        super()._run(args)

        for k, c in components:
            if k not in args["components"]:
                continue
            log.info("Building '{}'".format(k))
            c.build(args)


class CleanHandler(Handler):
    def __init__(self):
        super().__init__()

    def _create_parser(self):
        parser = argparse.ArgumentParser(
            description="Clean build files.",
            parents=[super()._create_parser()], add_help=False,
        )
        return parser
    
    def _handle_args(self, args):
        args = super()._handle_args(args)
        return args

    def _run(self, args):
        shutil.rmtree("build", ignore_errors=True)
        shutil.rmtree("target", ignore_errors=True)


class TestHandler(BuildHandler):
    def __init__(self):
        super().__init__()

    def _create_parser(self):
        parser = argparse.ArgumentParser(
            description="Test components.",
            parents=[super()._create_parser()], add_help=False,
        )
        parser.add_argument(
            "-a", "--args", action="append", type=str, default=[],
            help="Arguments to test."
        )
        return parser
    
    def _handle_args(self, args):
        args = super()._handle_args(args)
        return args

    def _run(self, args):
        super()._run(args)

        for k, c in components:
            if k not in args["components"]:
                continue
            log.info("Testing '{}'".format(k))
            c.test(args)


commands = [
    ("load",  LoadHandler()),
    ("build", BuildHandler()),
    ("clean", CleanHandler()),
    ("test",  TestHandler()),
]

command_parser = argparse.ArgumentParser(
    description="Hypertrace building and testing tool",
    usage="python3 -m manage <command> [options...]",
)
command_parser.add_argument(
    "command", choices=list(zip(*commands))[0],
    help=" ".join([
        "Task that will be performed by this script.",
        "To read about specific command use `<command> --help`"
    ]),
)

args = command_parser.parse_args(sys.argv[1:2])

dict(commands)[args.command](sys.argv[2:])
