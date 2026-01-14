# TDF Brand Identity & Design System

Complete brand guidelines for TrustDoc Financial (TDF) Desktop Viewer.

## Logo

The TDF logo represents **security, trust, and verification** through a shield containing a document with a lock and checkmark.

### Logo Elements

- **Shield**: Represents security and protection
- **Document**: Represents the TDF format
- **Lock**: Represents cryptographic security
- **Checkmark**: Represents verification and integrity

### Logo Colors

- **Primary Gradient**: Deep Blue (#1e40af) to Purple (#7c3aed)
- **Success Accent**: Green (#10b981) for verification checkmark

### Logo Files

- `icons/logo.svg` - Master SVG logo
- `icons/32x32.png` - Small icon
- `icons/128x128.png` - Standard icon
- `icons/128x128@2x.png` - Retina icon
- `icons/icon.ico` - Windows icon
- `icons/icon.icns` - macOS icon (create on macOS)
- `icons/icon.iconset/` - macOS iconset directory

## Color Palette

### Primary Colors

```css
--tdf-primary: #1e40af        /* Deep Blue - Trust, Security */
--tdf-primary-light: #3b82f6  /* Lighter Blue */
--tdf-primary-dark: #1e3a8a   /* Darker Blue */
```

**Usage**: Primary actions, headers, important elements

### Secondary Colors

```css
--tdf-secondary: #059669      /* Green - Success, Verification */
--tdf-secondary-light: #10b981
--tdf-secondary-dark: #047857
```

**Usage**: Success states, verification indicators, positive actions

### Accent Colors

```css
--tdf-accent: #7c3aed         /* Purple - Innovation */
--tdf-accent-light: #8b5cf6
--tdf-accent-dark: #6d28d9
```

**Usage**: Highlights, special features, gradients

### Neutral Colors

```css
--tdf-gray-50: #f9fafb        /* Lightest */
--tdf-gray-100: #f3f4f6
--tdf-gray-200: #e5e7eb
--tdf-gray-300: #d1d5db
--tdf-gray-400: #9ca3af
--tdf-gray-500: #6b7280
--tdf-gray-600: #4b5563
--tdf-gray-700: #374151
--tdf-gray-800: #1f2937
--tdf-gray-900: #111827        /* Darkest */
```

**Usage**: Backgrounds, borders, text, UI elements

### Semantic Colors

```css
--tdf-success: #10b981        /* Green */
--tdf-warning: #f59e0b         /* Amber */
--tdf-error: #ef4444          /* Red */
--tdf-info: #3b82f6           /* Blue */
```

## Typography

### Font Families

- **Primary**: System fonts (San Francisco, Segoe UI, Roboto)
- **Monospace**: Code, technical content

```css
--tdf-font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', ...
--tdf-font-mono: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Code', ...
```

## Spacing System

```css
--tdf-spacing-xs: 0.25rem     /* 4px */
--tdf-spacing-sm: 0.5rem      /* 8px */
--tdf-spacing-md: 1rem        /* 16px */
--tdf-spacing-lg: 1.5rem      /* 24px */
--tdf-spacing-xl: 2rem        /* 32px */
--tdf-spacing-2xl: 3rem       /* 48px */
```

## Border Radius

```css
--tdf-radius-sm: 0.25rem      /* 4px */
--tdf-radius-md: 0.5rem        /* 8px */
--tdf-radius-lg: 0.75rem       /* 12px */
--tdf-radius-xl: 1rem          /* 16px */
--tdf-radius-full: 9999px      /* Full circle */
```

## Shadows

```css
--tdf-shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05)
--tdf-shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1)
--tdf-shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
--tdf-shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1)
```

## Transitions

```css
--tdf-transition-fast: 150ms ease-in-out
--tdf-transition-base: 250ms ease-in-out
--tdf-transition-slow: 350ms ease-in-out
```

## Brand Gradient

The TDF brand uses a gradient from primary blue to accent purple:

```css
background: linear-gradient(135deg, #1e40af 0%, #7c3aed 100%);
```

**Usage**: Hero sections, buttons, highlights, logo backgrounds

## Logo Usage Guidelines

### Minimum Sizes

- **Small**: 32x32px (minimum for UI)
- **Standard**: 128x128px (recommended)
- **Large**: 512x512px (high-resolution displays)

### Clear Space

Maintain clear space around the logo equal to the height of the shield.

### Color Variations

- **Full Color**: Use on light backgrounds
- **Monochrome**: Use on colored backgrounds
- **Inverse**: White/light version for dark backgrounds

### Don'ts

- ❌ Don't stretch or distort the logo
- ❌ Don't rotate the logo
- ❌ Don't change the colors (except for monochrome/inverse)
- ❌ Don't place on busy backgrounds
- ❌ Don't use below minimum size

## Dark Mode

The design system supports dark mode with appropriate color adjustments:

- Backgrounds: Dark grays (#111827, #1f2937)
- Text: Light grays (#f9fafb, #d1d5db)
- Borders: Medium grays (#374151, #4b5563)

## Implementation

### CSS Variables

All brand colors and design tokens are available as CSS variables in `src/styles/brand.css`.

### Usage Example

```css
.button-primary {
  background-color: var(--tdf-primary);
  color: var(--tdf-text-inverse);
  border-radius: var(--tdf-radius-md);
  padding: var(--tdf-spacing-sm) var(--tdf-spacing-md);
  transition: all var(--tdf-transition-base);
}

.button-primary:hover {
  background-color: var(--tdf-primary-dark);
  box-shadow: var(--tdf-shadow-md);
}
```

## Icon Generation

### Regenerate Icons

```bash
cd tdf-desktop-viewer
python3 icons/create-icons-python.py
```

### Create ICNS on macOS

```bash
iconutil -c icns icons/icon.iconset -o icons/icon.icns
```

## Brand Assets

All brand assets are located in `tdf-desktop-viewer/icons/`:

- Logo SVG (master)
- PNG icons (various sizes)
- Windows ICO
- macOS ICNS (create on macOS)
- macOS iconset directory

## Color Psychology

- **Blue (#1e40af)**: Trust, security, professionalism, stability
- **Purple (#7c3aed)**: Innovation, creativity, technology
- **Green (#10b981)**: Success, verification, growth, safety

These colors work together to convey:
- **Security & Trust** (Blue)
- **Innovation & Technology** (Purple)
- **Verification & Success** (Green)

---

**Last Updated**: 2026-01-11  
**Version**: 1.0
