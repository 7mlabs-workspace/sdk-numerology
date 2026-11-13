"""Offline Pythagorean SDK. The bundled Rust library performs all calculations."""
import ctypes
import json
import platform
from pathlib import Path

__version__ = "1.1.0"
_machine = {"arm64": "arm64", "aarch64": "arm64", "x86_64": "x64", "AMD64": "x64"}.get(platform.machine())
_system = {"Darwin": "darwin", "Linux": "linux", "Windows": "win32"}.get(platform.system())
_ext = {"darwin": "dylib", "linux": "so", "win32": "dll"}.get(_system)
_library = Path(__file__).parent / "native" / f"{_system}-{_machine}" / f"libsevenmlabs_numerology.{_ext}"
if not _library.is_file():
    raise ImportError(f"No bundled Numerology native library for {platform.system()}/{platform.machine()}")
_lib = ctypes.CDLL(str(_library))
_lib.numerology_abi_version.restype = ctypes.c_uint32
if _lib.numerology_abi_version() != 1:
    raise ImportError("Numerology ABI version differs from wrapper")
_lib.numerology_version.restype = ctypes.c_char_p
if _lib.numerology_version().decode("ascii") != __version__:
    raise ImportError("Numerology engine version differs from wrapper")
_lib.numerology_calculate_json_len.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
_lib.numerology_calculate_json_len.restype = ctypes.c_void_p
_lib.numerology_calculate_json.argtypes = [ctypes.c_char_p]
_lib.numerology_calculate_json.restype = ctypes.c_void_p
_lib.numerology_calculate_filtered_json_len.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
_lib.numerology_calculate_filtered_json_len.restype = ctypes.c_void_p
_lib.numerology_free.argtypes = [ctypes.c_void_p]
_lib.numerology_free.restype = None

class NumerologyError(ValueError):
    def __init__(self, error):
        super().__init__(error["message"])
        self.code = error["code"]
        self.field = error["field"]

def calculate_json(request):
    """Return a JSON success/error envelope from a UTF-8 JSON string."""
    if not isinstance(request, str):
        raise TypeError("calculate_json requires a JSON string")
    encoded = request.encode("utf-8")
    pointer = _lib.numerology_calculate_json_len(encoded, len(encoded))
    if not pointer:
        raise MemoryError("Numerology returned a null result")
    try:
        return ctypes.string_at(pointer).decode("utf-8")
    finally:
        _lib.numerology_free(pointer)

def calculate(request):
    """Calculate from a dict; return result or raise NumerologyError."""
    envelope = json.loads(calculate_json(json.dumps(request, ensure_ascii=False, allow_nan=False)))
    if not envelope["ok"]:
        raise NumerologyError(envelope["error"])
    return envelope["result"]

def calculate_filtered_json(request):
    """Return a filtered-1.0 JSON envelope. Reduced disclosure does not mean anonymous."""
    if not isinstance(request, str):
        raise TypeError("calculate_filtered_json requires a JSON string")
    encoded = request.encode("utf-8")
    pointer = _lib.numerology_calculate_filtered_json_len(encoded, len(encoded))
    if not pointer:
        raise MemoryError("Numerology returned a null result")
    try:
        return ctypes.string_at(pointer).decode("utf-8")
    finally:
        _lib.numerology_free(pointer)

def calculate_filtered(request):
    """Opt-in filtered result from a dict; calculate keeps the original full output."""
    envelope = json.loads(calculate_filtered_json(json.dumps(request, ensure_ascii=False, allow_nan=False)))
    if not envelope["ok"]:
        raise NumerologyError(envelope["error"])
    return envelope["result"]
