import { TdfDocument, ContentBlock, TableBlock, DiagramBlock } from 'tdf-ts';
import { renderDiagram } from './diagram';

export function renderDocument(doc: TdfDocument, container: HTMLElement) {
    container.innerHTML = '';

    // Create document container
    const docContainer = document.createElement('div');
    docContainer.className = 'document-container';
    container.appendChild(docContainer);

    // Apply styles
    const styleSheet = document.createElement('style');
    styleSheet.textContent = doc.styles || getDefaultStyles();
    document.head.appendChild(styleSheet);

    // Render corporate header
    const headerEl = document.createElement('div');
    headerEl.className = 'document-header';
    headerEl.innerHTML = `
        <h1 class="document-title">${doc.manifest.document.title}</h1>
        <p class="document-subtitle">TrustDoc Financial Format - Confidential Document</p>
        <div class="security-badge">
            SECURED & VERIFIED
        </div>
    `;
    docContainer.appendChild(headerEl);

    // Create content area
    const contentEl = document.createElement('div');
    contentEl.className = 'document-content';
    docContainer.appendChild(contentEl);

    // Render metadata (corporate style)
    const metaEl = document.createElement('div');
    metaEl.className = 'document-meta';
    metaEl.innerHTML = `
        <p><strong>Document ID:</strong> <code>${doc.manifest.document.id}</code></p>
        <p><strong>Created:</strong> ${new Date(doc.manifest.document.created).toLocaleString()}</p>
        <p><strong>Modified:</strong> ${new Date(doc.manifest.document.modified).toLocaleString()}</p>
    `;
    contentEl.appendChild(metaEl);

    // Render authors
    if (doc.manifest.authors.length > 0) {
        const authorsEl = document.createElement('div');
        authorsEl.className = 'document-authors';
        authorsEl.innerHTML = '<strong>Authors:</strong> ' + 
            doc.manifest.authors.map(a => a.name).join(', ');
        contentEl.appendChild(authorsEl);
    }

    // Render sections
    for (const section of doc.content.sections) {
        const sectionEl = document.createElement('section');
        sectionEl.className = 'document-section';
        
        if (section.title) {
            const sectionTitle = document.createElement('h2');
            sectionTitle.textContent = section.title;
            sectionEl.appendChild(sectionTitle);
        }

        for (const block of section.content) {
            const blockEl = renderBlock(block);
            sectionEl.appendChild(blockEl);
        }

        contentEl.appendChild(sectionEl);
    }

    // Add footer
    const footerEl = document.createElement('div');
    footerEl.className = 'document-footer';
    footerEl.innerHTML = `
        <p>TrustDoc Financial Format - Confidential Business Document</p>
        <div class="security-note">
            This document is cryptographically secured. Any tampering will be immediately detectable.
        </div>
    `;
    docContainer.appendChild(footerEl);
}

function renderBlock(block: ContentBlock): HTMLElement {
    switch (block.type) {
        case 'heading':
            const h = document.createElement(`h${block.level}`);
            h.textContent = block.text;
            if (block.id) h.id = block.id;
            return h;

        case 'paragraph':
            const p = document.createElement('p');
            p.textContent = block.text;
            if (block.id) p.id = block.id;
            return p;

        case 'list':
            const list = document.createElement(block.ordered ? 'ol' : 'ul');
            for (const item of block.items) {
                const li = document.createElement('li');
                li.textContent = item;
                list.appendChild(li);
            }
            if (block.id) list.id = block.id;
            return list;

        case 'table':
            return renderTable(block);

        case 'diagram':
            return renderDiagram(block);

        case 'figure':
            const figure = document.createElement('figure');
            const img = document.createElement('img');
            img.src = block.asset;
            img.alt = block.alt;
            if (block.width) img.style.width = `${block.width}px`;
            figure.appendChild(img);
            if (block.caption) {
                const caption = document.createElement('figcaption');
                caption.textContent = block.caption;
                figure.appendChild(caption);
            }
            if (block.id) figure.id = block.id;
            return figure;

        case 'footnote':
            const fn = document.createElement('div');
            fn.className = 'footnote';
            fn.id = block.id;
            fn.textContent = block.text;
            return fn;

        default:
            const div = document.createElement('div');
            div.textContent = JSON.stringify(block);
            return div;
    }
}

function renderTable(table: TableBlock): HTMLElement {
    const tableEl = document.createElement('table');
    // Use both classes for compatibility
    tableEl.className = 'tdf-table';
    if (table.id) tableEl.id = table.id;

    // Header
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    for (const col of table.columns) {
        const th = document.createElement('th');
        th.textContent = col.header;
        headerRow.appendChild(th);
    }
    thead.appendChild(headerRow);
    tableEl.appendChild(thead);

    // Body
    const tbody = document.createElement('tbody');
    for (const row of table.rows) {
        const tr = document.createElement('tr');
        for (const col of table.columns) {
            const td = document.createElement('td');
            const cell = row[col.id];
            if (cell) {
                if ('display' in cell) {
                    td.textContent = (cell as any).display;
                } else if ('value' in cell) {
                    td.textContent = (cell as any).value;
                }
            }
            tr.appendChild(td);
        }
        tbody.appendChild(tr);
    }
    tableEl.appendChild(tbody);

    // Footer
    if (table.footer) {
        const tfoot = document.createElement('tfoot');
        const footerRow = document.createElement('tr');
        for (const cell of table.footer) {
            const td = document.createElement('td');
            td.textContent = cell;
            footerRow.appendChild(td);
        }
        tfoot.appendChild(footerRow);
        tableEl.appendChild(tfoot);
    }

    if (table.caption) {
        const caption = document.createElement('caption');
        caption.textContent = table.caption;
        tableEl.appendChild(caption);
    }

    return tableEl;
}

function getDefaultStyles(): string {
    return `
        .document-title { font-size: 24pt; font-weight: bold; margin-bottom: 1em; }
        .document-meta { margin-bottom: 1em; color: #666; }
        .document-authors { margin-bottom: 1em; }
        .document-section { margin-bottom: 2em; }
        .tdf-table { border-collapse: collapse; width: 100%; margin: 1em 0; }
        .tdf-table th, .tdf-table td { padding: 8px; border: 1px solid #ddd; }
        .tdf-table th { background-color: #f5f5f5; font-weight: bold; }
        .footnote { font-size: 0.9em; color: #666; margin-top: 0.5em; }
    `;
}

