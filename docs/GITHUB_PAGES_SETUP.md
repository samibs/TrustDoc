# GitHub Pages Setup Guide

## ✅ Setup Complete!

GitHub Pages has been configured for your TrustDoc repository.

## What Was Done

1. **Created GitHub Actions Workflow**
   - File: `.github/workflows/pages.yml`
   - Automatically deploys website on every push to `main` branch
   - Only triggers when `website/` directory changes

2. **Updated Website Links**
   - All links now point to your GitHub repository
   - Documentation links use GitHub blob URLs
   - Repository links updated to `samibs/TrustDoc`

## Enable GitHub Pages

**Important:** You need to enable GitHub Pages in repository settings:

1. Go to: https://github.com/samibs/TrustDoc/settings/pages
2. Under "Source", select: **GitHub Actions**
3. Click **Save**

**That's it!** The workflow will automatically deploy your website.

## Website URL

Once enabled, your website will be available at:

**https://samibs.github.io/TrustDoc/**

(It may take 1-2 minutes after the first push for GitHub to set it up)

## How It Works

1. **You push code** to `main` branch
2. **GitHub Actions** detects changes in `website/` directory
3. **Workflow runs** and deploys website
4. **Website updates** automatically at the URL above

## Custom Domain (Optional)

If you have a custom domain:

1. Go to repository Settings → Pages
2. Enter your domain in "Custom domain"
3. Configure DNS:
   - Add CNAME record pointing to `samibs.github.io`
   - Or A records pointing to GitHub IPs
4. Enable "Enforce HTTPS"

## Troubleshooting

### Website Not Showing

1. **Check Actions tab:**
   - Go to: https://github.com/samibs/TrustDoc/actions
   - Look for "Deploy GitHub Pages" workflow
   - Check if it completed successfully

2. **Check Pages settings:**
   - Go to: Settings → Pages
   - Make sure "Source" is set to "GitHub Actions"
   - Not "Deploy from a branch"

3. **Wait a few minutes:**
   - First deployment can take 2-5 minutes
   - Subsequent updates are faster

### Workflow Failing

- Check the Actions tab for error messages
- Common issues:
  - Missing permissions (should be auto-configured)
  - Syntax errors in workflow file
  - File path issues

## Manual Deployment

If you need to manually trigger deployment:

1. Go to: https://github.com/samibs/TrustDoc/actions
2. Select "Deploy GitHub Pages" workflow
3. Click "Run workflow"
4. Select branch: `main`
5. Click "Run workflow"

## Updating the Website

To update the website:

1. Edit files in `website/` directory
2. Commit changes
3. Push to `main` branch
4. Website updates automatically (usually within 1-2 minutes)

## Files Included

The website includes:
- `index.html` - Main page
- `styles.css` - Styling
- `script.js` - Interactivity

All other files in `website/` will also be deployed.

---

**Status:** ✅ Ready to deploy
**URL:** https://samibs.github.io/TrustDoc/ (after enabling Pages)

