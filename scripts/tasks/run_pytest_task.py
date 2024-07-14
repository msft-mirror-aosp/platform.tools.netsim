#!/usr/bin/env python3
#
# Copyright 2024 - The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the',  help="License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an',  help="AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import logging
from pathlib import Path
import platform

from environment import get_default_environment
from tasks.task import Task
from utils import (
    AOSP_ROOT,
    EMULATOR_ARTIFACT_PATH,
    binary_extension,
    run,
)

PYTEST_DIR = AOSP_ROOT / "external" / "adt-infra" / "pytest" / "test_embedded"
OBJS_DIR = AOSP_ROOT / "tools" / "netsim" / "objs"


class RunPyTestTask(Task):

  def __init__(self, args):
    super().__init__("RunPyTest")
    self.buildbot = args.buildbot
    self.pytest_input_dir = args.pytest_input_dir

  def do_run(self):
    run_pytest_manager = RunPytestManager(self.buildbot, self.pytest_input_dir)
    return run_pytest_manager.process()


class RunPytestManager:
  """Manager for running e2e integration pytests with Emulator

  The prerequisite is that the emulator installation has to be completed.
  RunPytestManager runs a run_tests shell script in external/adt-infra.
  It will take the emulator binary installed as the argument for the
  script.

  Attributes:

  buildbot: A boolean indicating if it's being invoked with Android Build
    Bots
  """

  def __init__(self, buildbot, pytest_input_dir):
    """Initializes the instances based on environment

    Args:
        buildbot: Defines if it's being invoked with Build Bots and defines
          self.dir as the directory of the emulator binary
        pytest_input_dir: Defined the directory that includes netsim and
          emulator binaries and libraries. Ignore if the string is empty.
    """
    # Default self.dir
    self.dir = EMULATOR_ARTIFACT_PATH / "emulator" if buildbot else OBJS_DIR

    # If pytest_input_dir is provided, set self.dir accordingly
    if pytest_input_dir:
      try:
        self.dir = AOSP_ROOT / "tools" / "netsim" / Path(pytest_input_dir)
      except Exception as e:
        logging.error(f"Invalid pytest_input_dir value: {e}")

  def _run_with_n_attempts(cmd, n):
    for attempt in range(1, n + 1):
      try:
        run(cmd, get_default_environment(AOSP_ROOT), "e2e_pytests")
        return
      except Exception as e:
        if attempt == n:
          raise e
        else:
          logging.error(f"PyTest Attempt {attempt} Error: {e}")

  def process(self) -> bool:
    """Process the emulator e2e pytests

    The process will check if the emulator installation occurred
    and run the run_tests.sh script.
    """
    emulator_bin = self.dir / binary_extension("emulator")
    if not (self.dir.exists() and emulator_bin.exists()):
      logging.info(
          "Please run 'scripts/build_tools.sh --InstallEmulator' "
          "before running RunPyTest"
      )
      return False
    if platform.system() == "Windows":
      run_tests_script = PYTEST_DIR / "run_tests.cmd"
    else:
      run_tests_script = PYTEST_DIR / "run_tests.sh"
    cmd = [
        run_tests_script,
        "--emulator",
        emulator_bin,
        "--test_config",
        PYTEST_DIR / "cfg" / "netsim_tests.json",
    ]
    # TODO: Resolve Windows PyTest failure
    if platform.system() != "Windows":
      cmd.append("--failures_as_errors")
    # Attempt rerunning test to resolve flakiness due to b/343503670
    if platform.system() == "Darwin" and platform.machine() == "arm64":
      RunPytestManager._run_with_n_attempts(cmd, 3)
    else:
      run(cmd, get_default_environment(AOSP_ROOT), "e2e_pytests")
    return True
