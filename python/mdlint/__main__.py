import subprocess
import sys
from pathlib import Path


def main() -> None:
    binary_name = "mdlint.exe" if sys.platform == "win32" else "mdlint"
    binary = Path(__file__).parent / binary_name

    if not binary.exists():
        print(
            f"error: mdlint binary not found at {binary}\n"
            "This may indicate a corrupted installation. Try reinstalling:\n"
            "  pip install --force-reinstall mdlint",
            file=sys.stderr,
        )
        sys.exit(1)

    result = subprocess.run([str(binary), *sys.argv[1:]])
    sys.exit(result.returncode)


if __name__ == "__main__":
    main()
