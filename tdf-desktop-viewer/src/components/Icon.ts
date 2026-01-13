// Icon Component - SVG Icon System
// Replaces emoji icons with accessible SVG icons

export type IconName = 
    | 'folder-open'
    | 'shield-check'
    | 'table-cells'
    | 'printer'
    | 'x-mark'
    | 'check-circle'
    | 'x-circle'
    | 'exclamation-triangle'
    | 'lock-closed'
    | 'document'
    | 'chevron-down'
    | 'chevron-up';

export type IconSize = 16 | 20 | 24;

interface IconProps {
    name: IconName;
    size?: IconSize;
    className?: string;
    ariaLabel?: string;
    ariaHidden?: boolean;
}

export function Icon({ name, size = 20, className = '', ariaLabel, ariaHidden = false }: IconProps): string {
    const iconPath = `/src/icons/${name}.svg`;
    const sizeClass = `icon-${size}`;
    const classes = `icon ${sizeClass} ${className}`.trim();
    
    // Return SVG element as string for innerHTML insertion
    // In a real component system, this would return an HTMLElement
    return `<img src="${iconPath}" 
                 class="${classes}" 
                 width="${size}" 
                 height="${size}" 
                 ${ariaLabel ? `aria-label="${ariaLabel}"` : ''} 
                 ${ariaHidden ? 'aria-hidden="true"' : ''}
                 alt="${ariaLabel || name}"
                 style="display: inline-block; vertical-align: middle;">`;
}

// Helper function to create icon element
export function createIconElement(name: IconName, size: IconSize = 20, ariaLabel?: string): HTMLElement {
    const img = document.createElement('img');
    img.src = `/src/icons/${name}.svg`;
    img.className = `icon icon-${size}`;
    img.width = size;
    img.height = size;
    img.style.display = 'inline-block';
    img.style.verticalAlign = 'middle';
    
    if (ariaLabel) {
        img.setAttribute('aria-label', ariaLabel);
        img.setAttribute('alt', ariaLabel);
    } else {
        img.setAttribute('aria-hidden', 'true');
    }
    
    return img;
}

// Load SVG content directly (better for styling)
export async function loadIconSVG(name: IconName): Promise<string> {
    try {
        const response = await fetch(`/src/icons/${name}.svg`);
        if (!response.ok) {
            throw new Error(`Failed to load icon: ${name}`);
        }
        return await response.text();
    } catch (error) {
        console.error(`Error loading icon ${name}:`, error);
        return '';
    }
}

// Inline SVG helper (for better styling control)
export async function createInlineIcon(name: IconName, size: IconSize = 20, className = '', ariaLabel?: string): Promise<HTMLElement> {
    const svgText = await loadIconSVG(name);
    if (!svgText) {
        // Fallback to img
        return createIconElement(name, size, ariaLabel);
    }
    
    const div = document.createElement('div');
    div.innerHTML = svgText;
    const svg = div.querySelector('svg') as SVGElement;
    
    if (svg) {
        svg.setAttribute('width', size.toString());
        svg.setAttribute('height', size.toString());
        svg.setAttribute('class', `icon icon-${size} ${className}`.trim());
        svg.setAttribute('fill', 'none');
        svg.setAttribute('stroke', 'currentColor');
        
        if (ariaLabel) {
            svg.setAttribute('aria-label', ariaLabel);
            svg.setAttribute('role', 'img');
        } else {
            svg.setAttribute('aria-hidden', 'true');
        }
        
        return svg;
    }
    
    return createIconElement(name, size, ariaLabel);
}
