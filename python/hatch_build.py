import os

from hatchling.builders.hooks.plugin.interface import BuildHookInterface


class CustomBuildHook(BuildHookInterface):
    """Inject a platform-specific wheel tag when MDLINT_PLATFORM_TAG is set.

    Without this hook, hatchling produces a pure-Python wheel (py3-none-any).
    Setting MDLINT_PLATFORM_TAG stamps the correct platform onto the wheel so
    that pip selects the right one at install time, e.g.:

        MDLINT_PLATFORM_TAG=macosx_11_0_arm64 uv build --wheel
    """

    def initialize(self, version: str, build_data: dict) -> None:
        platform_tag = os.environ.get("MDLINT_PLATFORM_TAG", "")
        if platform_tag:
            build_data["tag"] = f"py3-none-{platform_tag}"
