# Emu8080 Local Setup Instructions

This directory contains Stefan Tramm's Emu8080 CP/M emulator configured to run locally.

## Setup

The `emu8080/` directory contains the following files downloaded from https://st.sdf-eu.org/i8080/:

- `js8080.js` - Intel 8080 CPU emulation core
- `vt100.js` - VT100 terminal emulator
- `memio.js` - Memory and I/O handling
- `emulator.js` - Main emulator logic
- `styles.css` - Terminal styling
- `cpma.cpm` - CP/M disk image with Microsoft BASIC
- `websql-polyfill.js` - Our custom polyfill for WebSQL support in modern browsers

## WebSQL Polyfill

Modern browsers have deprecated WebSQL in favor of IndexedDB. The original Emu8080 relies on WebSQL for disk storage. To fix this, we've created `websql-polyfill.js` which:

1. Provides a compatibility layer that emulates WebSQL API
2. Uses in-memory storage for disk sectors
3. Persists data to localStorage when available
4. Handles all SQL operations needed by the emulator (CREATE TABLE, INSERT, SELECT, UPDATE, DELETE)

## Running the Emulator

1. Open `cpm-local.html` in your browser
2. The emulator will load automatically
3. You'll see the CP/M prompt (A> or similar)
4. Type `B:` to switch to drive B
5. Type `MBASIC` to start Microsoft BASIC-80

## Troubleshooting

### WebDB Not Supported Error
If you see "WebDB not supported" error:
- Check browser console for polyfill loading messages
- Ensure JavaScript is enabled
- Try a different browser (Chrome/Edge/Firefox recommended)

### Terminal Display Issues
If the terminal cursor doesn't align with typed text:
- The CSS fixes in emu8080.html force consistent font sizing
- Line height is set to 16px for proper spacing
- Auto-scroll is enabled via MutationObserver

### Text Not Scrolling
If terminal text builds up without scrolling:
- Auto-scroll JavaScript watches for content changes
- Scrolls to bottom on every keypress
- Custom scrollbar styling shows scroll position

## Files Not in Git

The `emu8080/` directory is in `.gitignore` because it contains third-party code. To set it up:

1. Download files from https://st.sdf-eu.org/i8080/
2. Place them in `emu8080/` directory
3. Copy the `websql-polyfill.js` from this documentation

## License

The original Emu8080 emulator is by Stefan Tramm, based on:
- js8080 from bluishcoder
- ShellInABox by Markus Gutschke
- Released under GPL v2

Our WebSQL polyfill is provided under the same BSD 3-Clause license as the rest of the MCDRAG project.