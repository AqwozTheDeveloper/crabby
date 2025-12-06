# ğŸ§ª Auto-Generated Test Results

Generated on: 12/06/2025 17:31:24

## âœ… PASS - TypeScript File
**Command**: ` cargo run --quiet -- run test.ts ` 
### Output
``nğŸ³ Cooking: npx -y ts-node test.ts
ğŸ½ï¸  Served! Done in 1s 486ms 859us 200ns

warning: unused import: `Serialize`
 --> src\package_utils.rs:1:26
  |
1 | ...e, Serialize};
  |       ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PathBuf`
 --> src\package_utils.rs:6:23
  |
6 | ...h, PathBuf};
  |       ^^^^^^^

warning: unused imports: `ProgressBar` and `ProgressStyle`
 --> src\runner.rs:4:17
  |
4 | ...::{ProgressBar, ProgressStyle};
  |       ^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `std::path::PathBuf`
 --> src\runner.rs:7:5
  |
7 | use std::path::PathBuf;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `ProgressBar` and `ProgressStyle`
 --> src\main.rs:9:17
  |
9 | ...::{ProgressBar, ProgressStyle};
  |       ^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused variable: `source_path`
   --> src\package_utils.rs:181:13
    |
181 | ...et source_path = ...
    |       ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_source_path`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
  --> src\runner.rs:27:9
   |
27 | ...et mut path_env = ...
   |       ----^^^^^^^^
   |       |
   |       help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default


` 

## âœ… PASS - JavaScript File
**Command**: ` cargo run --quiet -- run simple_test.js ` 
### Output
``nğŸ³ Cooking: node simple_test.js
Hello from simple test!
ğŸ½ï¸  Served! Done in 51ms 674us 400ns

warning: unused import: `Serialize`
 --> src\package_utils.rs:1:26
  |
1 | ...e, Serialize};
  |       ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PathBuf`
 --> src\package_utils.rs:6:23
  |
6 | ...h, PathBuf};
  |       ^^^^^^^

warning: unused imports: `ProgressBar` and `ProgressStyle`
 --> src\runner.rs:4:17
  |
4 | ...::{ProgressBar, ProgressStyle};
  |       ^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `std::path::PathBuf`
 --> src\runner.rs:7:5
  |
7 | use std::path::PathBuf;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `ProgressBar` and `ProgressStyle`
 --> src\main.rs:9:17
  |
9 | ...::{ProgressBar, ProgressStyle};
  |       ^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused variable: `source_path`
   --> src\package_utils.rs:181:13
    |
181 | ...et source_path = ...
    |       ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_source_path`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
  --> src\runner.rs:27:9
   |
27 | ...et mut path_env = ...
   |       ----^^^^^^^^
   |       |
   |       help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default


` 

## âœ… PASS - GUI Spawning
**Command**: ` cargo run --quiet -- run test_gui.ts ` 
### Output
``nâŒ Script 'test_gui.ts' not found in package.json. Available scripts: ["check-electron", "start-electron", "start", "shell-test"]

warning: unused import: `Serialize`
 --> src\package_utils.rs:1:26
  |
1 | ...e, Serialize};
  |       ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `PathBuf`
 --> src\package_utils.rs:6:23
  |
6 | ...h, PathBuf};
  |       ^^^^^^^

warning: unused imports: `ProgressBar` and `ProgressStyle`
 --> src\runner.rs:4:17
  |
4 | ...::{ProgressBar, ProgressStyle};
  |       ^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused import: `std::path::PathBuf`
 --> src\runner.rs:7:5
  |
7 | use std::path::PathBuf;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `ProgressBar` and `ProgressStyle`
 --> src\main.rs:9:17
  |
9 | ...::{ProgressBar, ProgressStyle};
  |       ^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: unused variable: `source_path`
   --> src\package_utils.rs:181:13
    |
181 | ...et source_path = ...
    |       ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_source_path`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
  --> src\runner.rs:27:9
   |
27 | ...et mut path_env = ...
   |       ----^^^^^^^^
   |       |
   |       help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default


` 


