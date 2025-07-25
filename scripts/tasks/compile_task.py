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

from pathlib import Path

from tasks.task import Task
from utils import (CMAKE, run)


class CompileTask(Task):

  def __init__(self, args, env):
    super().__init__("Compile")
    self.out = Path(args.out_dir)
    self.env = env

  def do_run(self):
    # Build
    run(
        [CMAKE, "--build", self.out],
        self.env,
        "bld",
    )
    return True
