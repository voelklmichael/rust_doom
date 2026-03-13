//
// config.h - Build configuration
//
// Original: config.hin.  Generated from configure.ac by autoheader.

// Define to 1 if you have the <dev/isa/spkrio.h> header file.
pub const HAVE_DEV_ISA_SPKRIO_H: bool = false;

// Define to 1 if you have the <dev/speaker/speaker.h> header file.
pub const HAVE_DEV_SPEAKER_SPEAKER_H: bool = false;

// Define to 1 if you have the <inttypes.h> header file.
pub const HAVE_INTTYPES_H: bool = true;

// Define to 1 if you have the `ioperm' function.
pub const HAVE_IOPERM: bool = false;

// Define to 1 if you have the `amd64' library (-lamd64).
pub const HAVE_LIBAMD64: bool = false;

// Define to 1 if you have the `i386' library (-li386).
pub const HAVE_LIBI386: bool = false;

// Define to 1 if you have the `m' library (-lm).
pub const HAVE_LIBM: bool = false;

// Define to 1 if you have the `png' library (-lpng).
pub const HAVE_LIBPNG: bool = false;

// Define to 1 if you have the `samplerate' library (-lsamplerate).
pub const HAVE_LIBSAMPLERATE: bool = false;

// Define to 1 if you have the `z' library (-lz).
pub const HAVE_LIBZ: bool = false;

// Define to 1 if you have the <linux/kd.h> header file.
pub const HAVE_LINUX_KD_H: bool = false;

// Define to 1 if you have the <memory.h> header file.
pub const HAVE_MEMORY_H: bool = false;

// Define to 1 if you have the `mmap' function.
pub const HAVE_MMAP: bool = false;

// Define to 1 if you have the `sched_setaffinity' function.
pub const HAVE_SCHED_SETAFFINITY: bool = false;

// Define to 1 if you have the <stdint.h> header file.
pub const HAVE_STDINT_H: bool = true;

// Define to 1 if you have the <stdlib.h> header file.
pub const HAVE_STDLIB_H: bool = true;

// Define to 1 if you have the <strings.h> header file.
pub const HAVE_STRINGS_H: bool = true;

// Define to 1 if you have the <string.h> header file.
pub const HAVE_STRING_H: bool = true;

// Define to 1 if you have the <sys/stat.h> header file.
pub const HAVE_SYS_STAT_H: bool = false;

// Define to 1 if you have the <sys/types.h> header file.
pub const HAVE_SYS_TYPES_H: bool = true;

// Define to 1 if you have the <unistd.h> header file.
pub const HAVE_UNISTD_H: bool = false;

// Name of package
pub const PACKAGE: &str = "Doom";

// Define to the address where bug reports for this package should be sent.
// (undef in original - use empty string when not defined)
pub const PACKAGE_BUGREPORT: &str = "";

// Define to the full name of this package.
pub const PACKAGE_NAME: &str = "Doom Generic";

// Define to the full name and version of this package.
pub const PACKAGE_STRING: &str = "Doom Generic 0.1";

// Define to the one symbol short name of this package.
pub const PACKAGE_TARNAME: &str = "doomgeneric.tar";

// Define to the home page for this package.
pub const PACKAGE_URL: &str = "";

// Define to the version of this package.
pub const PACKAGE_VERSION: &str = "0.1";

// Change this when you create your awesome forked version
pub const PROGRAM_PREFIX: &str = "doomgeneric";

// Define to 1 if you have the ANSI C header files.
pub const STDC_HEADERS: bool = true;

// Version number of package
pub const VERSION: &str = "0.1";

// Define to 1 if you want to compile the unmodified code
pub const ORIGCODE: bool = false;

// Define to the directory where all game files are located
pub const FILES_DIR: &str = ".";
