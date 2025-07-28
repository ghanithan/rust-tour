# Directory Browser Feature

## Overview
When the `download-exercises` feature is enabled, Rust Tour now offers users two ways to select the directory where exercises will be downloaded:

1. **Browse for directory** - Interactive directory navigation
2. **Enter path manually** - Traditional text input (original behavior)

## Implementation Details

### Feature Gate
All directory browser functionality is behind the `#[cfg(feature = "download-exercises")]` flag, ensuring it's only available when needed.

### New Function: `browse_for_directory`
Located in `web-server/src/main.rs`, this function provides:
- Interactive directory navigation using `dialoguer::Select`
- Options to select current directory, go up, browse subdirectories, or cancel
- Filters out hidden directories (starting with '.')
- Shows the current path at each step

### Updated Flow
1. User is prompted to download exercises
2. User chooses between "Browse for directory" or "Enter path manually"
3. If browse is selected:
   - Starts at user's home directory parent
   - User navigates to desired location
   - 'rust-tour-exercises' folder will be created at selection
4. If manual is selected:
   - Original text input with validation

### User Experience
```
📍 Choose where to store your exercises and progress:
   This will create a 'rust-tour-exercises' folder at your chosen location.
   Your progress will be saved here between sessions.

How would you like to select the directory?
> Browse for directory
  Enter path manually

[If Browse selected:]
📁 Navigate to your desired parent directory:
   The 'rust-tour-exercises' folder will be created inside your selection.

📂 Current: /home/username
> [✓ Select This Directory]
  [↑ Go Up]
  📁 Documents/
  📁 Downloads/
  📁 Projects/
  [✗ Cancel]
```

## Testing
Run with non-existent exercises path to trigger the download flow:
```bash
EXERCISES_PATH="/tmp/nonexistent" cargo run --package rust-tour --features "download-exercises"
```

Or use the test script:
```bash
./test_browser.sh
```