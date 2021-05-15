import os
import subprocess
import pytest
import sys
import logging


class OsmoseSystemTest:
    def __init__(self, release=False):
        self.project_root = os.path.normpath(os.path.abspath(__file__) + "/../../../")
        logging.info("Setting project root to %s", self.project_root)
        self.release_mode = release

        if sys.platform == "win32":
            self.extension = ".exe"
            logging.debug("Detected MS Windows as target platform")
        else:
            self.extension = ""

        self._rebuild()

        if release:
            self.binaries_dir = os.path.join(self.project_root, "target", "release")
        else:
            self.binaries_dir = os.path.join(self.project_root, "target", "debug")

        self.verbose = logging.DEBUG >= logging.root.level

    def run_binary(self, binary_name, *arguments):                        
        binary_name = binary_name + self.extension
        binary = os.path.join(self.binaries_dir, binary_name)
        if not os.path.isfile(binary):
            logging.error("Cannot find file %s in directory %s", binary_name, self.binaries_dir)
            raise RuntimeError("Cannot find requested file!")
        logging.debug("Starting binary `%s` with arguments: %s", binary, " ".join(arguments))
        env = os.environ
        if self.verbose:
            env["RUST_BACKTRACE"] = "1"
        return subprocess.Popen([binary, *arguments], env=env)

    def _rebuild(self):
        cargo_args = ["cargo" + self.extension, "build"]
        result = subprocess.run(cargo_args, cwd=self.project_root, capture_output=True)
        if result.returncode != 0:
            logging.debug(result.stdout)
            logging.error(result.stderr)
            raise RuntimeError("Cannot rebuild project!")
        logging.debug("Rebuild finished successfully")

