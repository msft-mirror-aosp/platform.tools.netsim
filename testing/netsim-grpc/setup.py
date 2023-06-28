"""
    Setup file for netsim-grpc.
    Use setup.cfg to configure your project.

    This file was generated with PyScaffold 4.3.
    PyScaffold helps you to put up the scaffold of your new Python project.
    Learn more under: https://pyscaffold.org/
"""
import os
import subprocess
import sys
from os import path

from setuptools import setup
from setuptools.command.build_py import build_py


class ProtoBuild(build_py):
    """
    This command automatically compiles all netsim .proto files with `protoc` compiler
    and places generated files under src/netsim/proto
    """

    def run(self):
        here = path.abspath(path.dirname(__file__))
        root_dir = path.dirname(path.dirname(here))
        proto_dir = path.join(root_dir, "proto")
        out_dir = path.join(here, "src", "netsim", "proto")

        for proto_file in filter(
            lambda x: x.endswith(".proto"), os.listdir(path.join(proto_dir, "netsim"))
        ):
            source = path.join(proto_dir, "netsim", proto_file)
            output = path.join(out_dir, "netsim", proto_file).replace(".proto", "_pb2.py")
            output_grpc = path.join(out_dir, "netsim", proto_file).replace(".proto", "_pb2_grpc.py")

            if not path.exists(output) or (
                path.getmtime(source) > path.getmtime(output)
            ):
                sys.stderr.write(f"Protobuf-compiling {source}\n")

                subprocess.check_call(
                    [
                        sys.executable,
                        "-m",
                        "grpc_tools.protoc",
                        f"-I{proto_dir}",
                        f"--python_out={out_dir}",
                        f"--grpc_python_out={out_dir}",
                        source,
                    ]
                )

                # Patch import in the generated grpc and protobuf backends.
                # The default plugin will generate 'import "netsim/common.proto"'
                # as 'from netsim import common' but that generates many
                # issues for nested modules.
                # Replace these import statements with a local import:
                # 'from . import common' to ensure that the generated files can be
                # relocated.
                def patch_import(file_name):
                    with open(file_name, 'r') as f:
                        contents = f.read()
                    contents = contents.replace("from netsim import", "from . import")
                    with open(file_name, 'w') as f:
                        f.write(contents)

                patch_import(output)
                patch_import(output_grpc)

        super().run()


if __name__ == "__main__":
    try:
        setup(
            # use_scm_version={"version_scheme": "no-guess-dev", "root": "../../../"},
            cmdclass={"build_py": ProtoBuild},
        )
    except:  # noqa
        print(
            "\n\nAn error occurred while building the project, "
            "please ensure you have the most updated version of setuptools, "
            "setuptools_scm and wheel with:\n"
            "   pip install -U setuptools setuptools_scm wheel\n\n"
        )
        raise
