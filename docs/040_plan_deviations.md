# Plan Deviations Report

This document lists deviations between the actual Stage 0 migration and the rules in [010_basic_plan.md](010_basic_plan.md). Use this to guide corrections or a restart.

---

## 1. Headers Only vs. Paired Modules

**Plan:** For paired modules (`x.h` + `x.c`), create a single `x.rs` with:
- Declarations from `.h`
- **Implementations from `.c`** (globals, function bodies as comments, etc.)

**Deviation:** Migration focused on `.h` content. `.c` content was often not migrated:
- Function bodies in `.c` → mostly skipped (see section 2)
- Static/global variables in `.c` → often missing (see section 5)
- Static arrays/const data in `.c` → sometimes missing (e.g. `rndtable` in m_random)

**Examples:**
- `m_random.c`: Contains `rndtable[256]`; `m_random.rs` has no equivalent.
- `v_video.c`: Has `xlatab`, `dest_screen`, `patchclip_callback`; `v_video.rs` only has `tinttable`, `dirtybox`.
- `m_fixed.c`: FixedDiv has full `if/else` logic; comment in Rust only shows the `else` branch.

---

## 2. Missing C Function Bodies in Comments

**Plan (§2.8):** For each function:
1. Copy the C function body as a comment.
2. Implement Rust body as `todo!("Basic stage-0 stub")`.

**Deviation:** Most functions lack the C body comment. Only a minority of stubs include `// C body:`.

**Evidence:**
- ~11 modules have at least one `// C body` comment.
- 130+ functions use `todo!("Basic stage-0 stub")` without the corresponding C body.

**Examples with C body (correct):**
- `m_fixed.rs`: FixedMul, FixedDiv
- `i_swap.rs`: swap_le16, swap_le32
- `dummy.rs`: I_InitTimidityConfig

**Examples missing C body:**
- `m_random.rs`: M_Random, P_Random, M_ClearRandom
- `v_video.rs`: V_MarkRect (plan example in 2.8 shows this *should* have the C body)
- `w_merge.rs`: All four functions
- Most paired modules

---

## 3. Incomplete Constants (especially string headers)

**Plan (§2.2):** `#define FOO 42` → `pub static FOO: i32 = 42` (or appropriate type). Add comment with original macro name.

**Deviation:** Many header files define dozens or hundreds of constants; Rust modules define only a subset.

### 3.1 d_englsh (severe)

| Source | Count |
|--------|-------|
| `d_englsh.h` | **286** `#define` macros |
| `d_englsh.rs` | **~16** constants |

**Missing (examples):** LOADNET, QLOADNET, QSAVESPOT, SAVEDEAD, QSPROMPT, QLPROMPT, NETEND, ENDGAME, DOSY, GAMMALVL0–4, GOTARMBONUS, GOTSTIM, GOTMEDINEED, GOTMEDIKIT, GOTSUPER, GOTBLUECARD, GOTYELWCARD, GOTREDCARD, GOTBLUESKUL, GOTYELWSKUL, GOTREDSKULL, GOTINVUL, GOTBERSERK, GOTINVIS, GOTSUIT, GOTMAP, GOTVISOR, GOTMSPHERE, GOTCLIP, GOTCLIPBOX, GOTROCKET, GOTROCKBOX, GOTCELL, GOTCELLBOX, GOTSHELLS, GOTSHELLBOX, GOTBACKPACK, GOTBFG9000, GOTCHAINGUN, GOTCHAINSAW, GOTLAUNCHER, GOTPLASMA, GOTSHOTGUN, GOTSHOTGUN2, PD_BLUEO, PD_REDO, PD_YELLOWO, PD_BLUEK, PD_REDK, PD_YELLOWK, GGSAVED, HUSTR_*, all level names (E1M1–E4M9, etc.), all menu strings, all cheat strings, etc.

### 3.2 config.h

- Migrated: PACKAGE, PACKAGE_NAME, FILES_DIR
- Missing: PACKAGE_STRING, PACKAGE_TARNAME, PACKAGE_URL, PACKAGE_VERSION, PROGRAM_PREFIX, VERSION, HAVE_* (if used per plan 4.3), STDC_HEADERS, etc.

### 3.3 Other modules

Similar incompleteness likely in: `deh_misc`, `doomkeys`, `doomdata`, `d_event`, `p_mobj`, etc. A full audit would require comparing each `.h` to its `.rs`.

---

## 4. Plan Says `pub static`, Code Uses `pub const`

**Plan (§2.2):** `#define FOO 42` → `pub static FOO: i32 = 42;`  
Example in plan: `pub static SCREENWIDTH: i32 = 320;`

**Deviation:** Most constants use `pub const` instead of `pub static`.

**Examples:** `config.rs`, `d_englsh.rs`, `m_fixed.rs`, `deh_main.rs`, etc.

**Impact:** Minor. In Rust, `const` vs `static` for compile-time integer/string literals is usually equivalent for read-only use. Plan might be updated if `const` is preferred.

---

## 5. Missing Globals / State Fields

**Plan (§2.7):** Every global and `extern` must be in `X_State` as `Arc<Mutex<T>>`. For `extern`, the owning module provides a getter with `// extern <c-variable-name>`.

**Deviation:** Globals from `.c` files are often missing from `*_State`.

**Examples:**
- `m_random.c`: `rndtable[256]` — not in `M_RandomState`. Plan would map this to a const array or state field.
- `v_video.c`: `xlatab`, `dest_screen`, `patchclip_callback` — not in `V_VideoState`.
- `deh_main.h`: `extern boolean deh_allow_extended_strings`, `deh_allow_long_strings`, `deh_allow_long_cheats`, `deh_apply_cheats` — not in `DehMainState`.
- `dummy.c`: `net_client_connected`, `drone` — present in `DummyState` ✓

---

## 6. Missing Declarations (Functions, Types, Macros)

**Plan:** Each `.h` declaration (function, type, macro) should have a Rust counterpart.

**Deviation:** Several header-only or paired modules omit declarations.

**Examples:**
- **deh_str.h:** When FEATURE_DEHACKED is disabled, provides macros `DEH_String(x)`, `DEH_printf`, etc. Rust `deh_str.rs` is effectively empty — no pass-through API.
- **v_video.h:** Declares V_SetPatchClipCallback, V_DrawAltTLPatch, V_DrawShadowedPatch, V_DrawXlaPatch, V_DrawFilledBox, V_DrawHorizLine, V_DrawVertLine, V_DrawBox, V_DrawRawScreen, V_ScreenShot, V_LoadXlaTable, V_DrawMouseSpeedBox — not all present in `v_video.rs`.
- **i_swap.h:** `SHORT(x)`, `LONG(x)`, `doom_swap_s`, `doom_wtohs` macros — not migrated (plan 4 says SYS_LITTLE_ENDIAN only; these may be big-endian paths to remove).

---

## 7. C-Only Modules: Implementation Content

**Plan (§1.3):** For `.c`-only modules, migrate implementation content (globals, functions).

**Deviation:** C-only modules may have minimal migration. Example `dummy.c` is correctly migrated (globals, one function), but others (e.g. `p_map.c`, `p_floor.c`) would need a pass to ensure all functions and globals are present with C bodies in comments.

---

## 8. Include Mapping (`#include "y.h"` → `pub use y::*`)

**Plan (§2.1):** `#include "y.h"` in x.h → `pub use y::*` in x.rs.

**Deviation:** Not systematically applied. Many Rust modules use `use crate::y::*` or specific imports instead of re-exporting. `r_local.rs` correctly re-exports from tables, doomdef, r_data, r_main, etc. Others (e.g. `v_video.rs`) only `use` what they need and don’t re-export. Plan may intend re-exports only where the C header is included for API surface; clarification needed.

---

## 9. Incomplete C Body Comments (Truncated Logic)

**Plan:** Copy the *full* C function body as a comment.

**Deviation:** Some comments show only part of the body.

**Example:**
- `m_fixed.rs` `FixedDiv`: Comment shows `((int64_t) a << 16) / b` and the return, but omits the `if ((abs(a) >> 14) >= abs(b))` branch and INT_MIN/INT_MAX handling.

---

## 10. Feature Switches / Conditional Code

**Plan (§4):** Used features (SYS_LITTLE_ENDIAN, RANGECHECK, ORIGCODE, DOOM_GENERIC) — include C code directly. Unused features — remove.

**Deviation:** Some modules may include declarations or constants from disabled-feature paths, or omit used-feature paths. No systematic audit has been done.

---

## Summary Table

| Deviation Type | Severity | Estimated Effort |
|----------------|----------|------------------|
| 1. Headers only, .c skipped | High | Full re-pass on paired modules |
| 2. Missing C body comments | High | ~130+ functions to annotate |
| 3. Incomplete constants | High | d_englsh alone: 270+ constants; others TBD |
| 4. const vs static | Low | Optional, style-only |
| 5. Missing globals | High | Per-module audit |
| 6. Missing declarations | Medium | Per-header audit |
| 7. C-only modules | Medium | Per-module audit |
| 8. Include mapping | Low–Medium | Clarify intent, then apply |
| 9. Truncated C bodies | Medium | Fix known cases, then scan |
| 10. Feature switches | Medium | Audit conditional blocks |

---

## Recommended Next Steps

1. **Prioritise d_englsh:** Migrate all 286 constants (or document why some are excluded).
2. **Audit a few paired modules end-to-end:** e.g. `m_random`, `m_fixed`, `v_video` — header + .c vs. .rs, line by line.
3. **Add C body comments:** Script or manual pass to copy C bodies into every `todo!` stub.
4. **Add missing globals:** Ensure every global/extern from `.c`/`.h` appears in `*_State`.
5. **Decide on const vs static:** Update plan or code for consistency.
6. **Create a checklist per module:** Plan-compliant vs. actual content.
