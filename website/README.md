# TDF Website

Professional website for the TrustDoc Financial (TDF) format.

## Features

- Modern, responsive design
- Mobile-friendly
- Fast loading
- SEO optimized
- Clear value proposition
- Call-to-actions
- Documentation links

## Structure

```
website/
├── index.html      # Main page
├── styles.css     # Styling
├── script.js      # Interactivity
└── README.md      # This file
```

## Deployment

### Option 1: GitHub Pages

1. Push website files to `gh-pages` branch
2. Enable GitHub Pages in repository settings
3. Website will be available at `https://username.github.io/tdf/`

### Option 2: Netlify

1. Connect repository to Netlify
2. Set build directory to `website`
3. Deploy automatically on push

### Option 3: Vercel

1. Connect repository to Vercel
2. Set root directory to `website`
3. Deploy automatically on push

### Option 4: Static Hosting

Upload files to any static hosting service:
- AWS S3 + CloudFront
- Google Cloud Storage
- Azure Static Web Apps
- Any web server

## Customization

### Colors

Edit CSS variables in `styles.css`:

```css
:root {
    --primary-color: #1a237e;
    --primary-dark: #0d47a1;
    --accent-color: #00acc1;
    /* ... */
}
```

### Content

Edit `index.html` to update:
- Text content
- Links
- Sections
- Features

### Links

Update links to point to:
- GitHub repository
- Documentation
- Social media
- Contact information

## Development

### Local Testing

```bash
cd website
python3 -m http.server 8000
# Or
npx serve .
```

Open `http://localhost:8000` in browser.

### Build

No build step required - pure HTML/CSS/JS.

## SEO

The website includes:
- Meta tags (description, keywords)
- Semantic HTML
- Proper heading structure
- Alt text for images (add when adding images)
- Open Graph tags (add if needed)

## Performance

- No external dependencies
- Minimal JavaScript
- Optimized CSS
- Fast loading

## Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Mobile browsers

## License

Same as main project: MIT OR Apache-2.0

