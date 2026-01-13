// Button Component System
// Enterprise-grade button component with semantic variants and states

import { createInlineIcon, IconName } from './Icon';

export type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'ghost';
export type ButtonSize = 'sm' | 'md' | 'lg';

interface ButtonOptions {
    variant?: ButtonVariant;
    size?: ButtonSize;
    icon?: IconName;
    iconPosition?: 'left' | 'right';
    loading?: boolean;
    disabled?: boolean;
    fullWidth?: boolean;
    ariaLabel?: string;
    onClick?: (e: MouseEvent) => void;
    className?: string;
}

export class Button {
    private element: HTMLButtonElement;
    private options: Required<Omit<ButtonOptions, 'onClick' | 'ariaLabel'>> & Pick<ButtonOptions, 'onClick' | 'ariaLabel'>;

    constructor(text: string, options: ButtonOptions = {}) {
        this.options = {
            variant: options.variant || 'primary',
            size: options.size || 'md',
            icon: options.icon,
            iconPosition: options.iconPosition || 'left',
            loading: options.loading || false,
            disabled: options.disabled || false,
            fullWidth: options.fullWidth || false,
            ariaLabel: options.ariaLabel,
            onClick: options.onClick,
            className: options.className || ''
        };

        this.element = this.create();
        this.updateText(text);
    }

    private create(): HTMLButtonElement {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = this.getClassName();
        
        if (this.options.ariaLabel) {
            button.setAttribute('aria-label', this.options.ariaLabel);
        }
        
        if (this.options.disabled || this.options.loading) {
            button.disabled = true;
            button.setAttribute('aria-disabled', 'true');
        }
        
        button.setAttribute('tabindex', '0');
        
        if (this.options.onClick) {
            button.addEventListener('click', (e) => {
                if (!this.options.disabled && !this.options.loading) {
                    this.options.onClick!(e);
                }
            });
        }

        return button;
    }

    private getClassName(): string {
        const classes = [
            'btn',
            `btn-${this.options.variant}`,
            `btn-${this.options.size}`
        ];

        if (this.options.loading) {
            classes.push('btn-loading');
        }

        if (this.options.disabled) {
            classes.push('btn-disabled');
        }

        if (this.options.fullWidth) {
            classes.push('btn-full-width');
        }

        if (this.options.className) {
            classes.push(this.options.className);
        }

        return classes.join(' ');
    }

    async updateText(text: string): Promise<void> {
        // Clear existing content
        this.element.innerHTML = '';

        // Add loading spinner if loading
        if (this.options.loading) {
            const spinner = document.createElement('span');
            spinner.className = 'btn-spinner';
            spinner.setAttribute('aria-hidden', 'true');
            this.element.appendChild(spinner);
            this.element.appendChild(document.createTextNode(' '));
        }

        // Add icon if specified
        if (this.options.icon && !this.options.loading) {
            const iconContainer = document.createElement('span');
            iconContainer.className = `btn-icon btn-icon-${this.options.iconPosition}`;
            const icon = await createInlineIcon(this.options.icon, this.getIconSize(), '', '');
            iconContainer.appendChild(icon);
            this.element.appendChild(iconContainer);
        }

        // Add text
        const textSpan = document.createElement('span');
        textSpan.className = 'btn-text';
        textSpan.textContent = text;
        this.element.appendChild(textSpan);
    }

    private getIconSize(): 16 | 20 | 24 {
        switch (this.options.size) {
            case 'sm': return 16;
            case 'md': return 20;
            case 'lg': return 24;
        }
    }

    setLoading(loading: boolean): void {
        this.options.loading = loading;
        if (loading) {
            this.element.disabled = true;
            this.element.classList.add('btn-loading');
        } else {
            this.element.disabled = this.options.disabled;
            this.element.classList.remove('btn-loading');
        }
        this.updateText(this.element.querySelector('.btn-text')?.textContent || '');
    }

    setDisabled(disabled: boolean): void {
        this.options.disabled = disabled;
        this.element.disabled = disabled || this.options.loading;
        if (disabled) {
            this.element.classList.add('btn-disabled');
            this.element.setAttribute('aria-disabled', 'true');
        } else {
            this.element.classList.remove('btn-disabled');
            this.element.removeAttribute('aria-disabled');
        }
    }

    getElement(): HTMLButtonElement {
        return this.element;
    }

    // Static factory methods for convenience
    static async create(text: string, options: ButtonOptions = {}): Promise<Button> {
        const button = new Button(text, options);
        return button;
    }

    static createPrimary(text: string, onClick?: (e: MouseEvent) => void): Button {
        return new Button(text, { variant: 'primary', onClick });
    }

    static createSecondary(text: string, onClick?: (e: MouseEvent) => void): Button {
        return new Button(text, { variant: 'secondary', onClick });
    }

    static createDanger(text: string, onClick?: (e: MouseEvent) => void): Button {
        return new Button(text, { variant: 'danger', onClick });
    }
}

// Helper function to create button element directly
export async function createButtonElement(
    text: string,
    options: ButtonOptions = {}
): Promise<HTMLButtonElement> {
    const button = new Button(text, options);
    return button.getElement();
}
