# CSS Loading Fix for Fly.io Deployment

## Problem
CSS was not being loaded in the dockerized container when deployed to Fly.io, even though the CSS files were present in the container.

## Root Cause
The issue was in the `web/Dioxus.toml` configuration file. The `style` array was empty:

```toml
[web.resource]
# Additional CSS style files
style = []
```

This meant that while the CSS files existed in the container and were properly copied by the Dockerfile, **Dioxus was not including them in the generated web bundle**. The Dioxus build process needs to be explicitly told which CSS files to include.

## Solution
Updated `web/Dioxus.toml` to include all CSS files in the `style` array:

```toml
[web.resource]
# Additional CSS style files
style = [
    "assets/main.css",
    "assets/components.css", 
    "assets/blog.css",
    "assets/markdown.css",
    "assets/syntax.css"
]
```

## How It Works

### CSS File Flow
1. **Source Files**: CSS files exist in multiple source directories:
   - `ba-server/assets/` - Contains main styling files
   - `ui/assets/` - Contains UI-specific styles  
   - `web/assets/` - Contains web platform styles

2. **Docker Build**: The Dockerfile correctly copies all CSS files:
   ```dockerfile
   COPY --from=builder /app/ba-server/assets/ /usr/local/app/assets/
   COPY --from=builder /app/ui/assets/ /usr/local/app/assets/
   ```

3. **Dioxus Build**: When `dx bundle` runs, it now includes the CSS files specified in `Dioxus.toml`

4. **Runtime**: The server serves assets from `/usr/local/app/assets/` via the `/assets` route

### Path Resolution
- **Local development**: Server checks for `assets/` directory, falls back to `ba-server/assets/`
- **Docker/Fly.io**: Server finds `assets/` directory at `/usr/local/app/assets/` (working directory is `/usr/local/app`)

## Why This Fixes Fly.io Deployment
- **Before**: CSS files existed in container but weren't included in the Dioxus web bundle
- **After**: CSS files are explicitly included in the Dioxus build and served to browsers
- **Result**: Browsers can successfully load CSS from `/assets/` URLs

## Validation
Run the test script to verify the fix:
```bash
./test_css_fix.sh
```

## Testing the Fix
1. **Local Build**: `docker compose build`
2. **Deploy**: `fly deploy`
3. **Verify**: Check browser dev tools to confirm CSS files load successfully
4. **Visual Check**: Confirm styling is applied correctly to the website

## Files Changed
- `web/Dioxus.toml` - Added CSS files to `style` array

## Technical Notes
- Dioxus uses the `[web.resource]` section to determine which assets to include in web builds
- The `style` array must contain paths relative to the assets directory
- All CSS files are now included: main, components, blog, markdown, and syntax highlighting styles